#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use codec::{Decode, Encode};
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{
            Currency, ExistenceRequirement, LockableCurrency, Randomness, ReservableCurrency,
        },
    };
    use frame_system::pallet_prelude::*;
    use sp_io::hashing::blake2_128;
    use sp_runtime::traits::{AtLeast32BitUnsigned, Bounded, Member, One};

    #[derive(Encode, Decode, Debug, PartialEq)]
    pub struct Kitty(pub [u8; 16]);

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
        type KittyIndex: Parameter + Member + AtLeast32BitUnsigned + Bounded + One + Default + Copy;
        type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>
            + ReservableCurrency<Self::AccountId>;
        #[pallet::constant]
        type MinimumVotingLock: Get<BalanceOf<Self>>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn kitties_count)]
    pub type KittiesCount<T: Config> = StorageValue<_, T::KittyIndex>;

    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub type Kitties<T: Config> =
        StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<Kitty>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn owner)]
    pub type Owner<T: Config> =
        StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn kitties_market)]
    pub type KittiesMarket<T: Config> =
        StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<BalanceOf<T>>, ValueQuery>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        KittyCreate(T::AccountId, T::KittyIndex),
        KittyTransfer(T::AccountId, T::AccountId, T::KittyIndex),
        KittyBreed(T::AccountId, T::KittyIndex),
        KittyMarket(T::AccountId, T::KittyIndex, BalanceOf<T>),
        KittyBuy(T::AccountId, T::KittyIndex, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        KittiesCountOverflow,
        NotOwner,
        SameParentIndex,
        InvalidKittyIndex,
        InvalidMarketPrice,
        PriceTooLow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn create(origin: OriginFor<T>) -> DispatchResult {
            // 方法调用者
            let sender = ensure_signed(origin)?;

            // 下一个id
            let kitty_id = Self::next_kitty_id()?;

            // 创建dna
            let dna = Self::random_value(&sender);

            // 锁定一定的钱
            T::Currency::reserve(&sender, T::MinimumVotingLock::get())?;

            // 保存新的kitty
            Self::add_one_kitty(sender.clone(), kitty_id, dna);

            // 事件
            Self::deposit_event(Event::KittyCreate(sender, kitty_id));

            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn transfer(
            origin: OriginFor<T>,
            new_owner: T::AccountId,
            kitty_id: T::KittyIndex,
        ) -> DispatchResult {
            // 方法调用者
            let sender = ensure_signed(origin)?;

            // kitty是调用者的
            ensure!(
                Some(sender.clone()) == Owner::<T>::get(kitty_id),
                Error::<T>::NotOwner
            );

            // 变一下kitty所属关系
            Owner::<T>::insert(kitty_id, Some(new_owner.clone()));

            // 事件
            Self::deposit_event(Event::KittyTransfer(sender, new_owner, kitty_id));

            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn breed(
            origin: OriginFor<T>,
            kitty_id_1: T::KittyIndex,
            kitty_id_2: T::KittyIndex,
        ) -> DispatchResult {
            // 方法调用者
            let sender = ensure_signed(origin)?;

            // 获取孩子的id
            let kitty_id = Self::next_kitty_id()?;

            // 生成孩子的dna
            let dna = Self::breed_dna(&sender, kitty_id_1, kitty_id_2)?;

            // 保存孩子kitty
            Self::add_one_kitty(sender.clone(), kitty_id, dna);

            // 事件
            Self::deposit_event(Event::KittyBreed(sender, kitty_id));

            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn market(
            origin: OriginFor<T>,
            kitty_id: T::KittyIndex,
            price: BalanceOf<T>,
        ) -> DispatchResult {
            // 方法调用者
            let sender = ensure_signed(origin)?;

            // kitty是调用者的
            ensure!(
                Some(sender.clone()) == Owner::<T>::get(kitty_id),
                Error::<T>::NotOwner
            );

            // 把kitty放到市场上
            KittiesMarket::<T>::insert(kitty_id, Some(price));

            // 事件
            Self::deposit_event(Event::KittyMarket(sender, kitty_id, price));

            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn buy(
            origin: OriginFor<T>,
            kitty_id: T::KittyIndex,
            price: BalanceOf<T>,
        ) -> DispatchResult {
            // 方法调用者
            let sender = ensure_signed(origin)?;

            // 从市场购买kitty
            let kitty_price = Self::buy_kitty(&sender, kitty_id, price)?;

            // 事件
            Self::deposit_event(Event::KittyBuy(sender, kitty_id, kitty_price));

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        // 使用随机数创建一个dna
        fn random_value(sender: &T::AccountId) -> [u8; 16] {
            let payload = (
                T::Randomness::random_seed(),
                &sender,
                <frame_system::Pallet<T>>::extrinsic_index(),
            );
            payload.using_encoded(blake2_128)
        }

        // 获取下一个id
        fn next_kitty_id() -> sp_std::result::Result<T::KittyIndex, DispatchError> {
            let kitty_id = match Self::kitties_count() {
                Some(id) => {
                    // 溢出报错
                    ensure!(
                        id != T::KittyIndex::max_value(),
                        Error::<T>::KittiesCountOverflow
                    );
                    // 获取下一个id的时候加1，存的时候就不加了
                    id + T::KittyIndex::one()
                }
                // 第一次获取的时候直接用1
                None => T::KittyIndex::one(),
            };
            Ok(kitty_id)
        }

        // 增加一个kitty
        fn add_one_kitty(owner: T::AccountId, kitty_id: T::KittyIndex, dna: [u8; 16]) {
            // 保存kitty
            Kitties::<T>::insert(kitty_id, Some(Kitty(dna)));
            // 保存kitty属于谁
            Owner::<T>::insert(kitty_id, Some(owner.clone()));
            // 创建了多少个kitty了
            KittiesCount::<T>::put(kitty_id);
        }

        // 生成孩子的dna
        fn breed_dna(
            sender: &T::AccountId,
            kitty_id_1: T::KittyIndex,
            kitty_id_2: T::KittyIndex,
        ) -> sp_std::result::Result<[u8; 16], DispatchError> {
            // 父母不能一致
            ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameParentIndex);

            // 通过id获取父母
            let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyIndex)?;
            let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyIndex)?;

            // 父母的dna
            let dna_1 = kitty1.0;
            let dna_2 = kitty2.0;

            // 通过父母的dna生成孩子的dna
            let selector = Self::random_value(sender);
            let mut new_dna = [0u8; 16];
            for i in 0..dna_1.len() {
                new_dna[i] = (selector[i] & dna_1[i]) | (!selector[i] & dna_2[i]);
            }

            // 返回新的dna
            Ok(new_dna)
        }

        // 从市场购买kitty
        fn buy_kitty(
            sender: &T::AccountId,
            kitty_id: T::KittyIndex,
            price: BalanceOf<T>,
        ) -> sp_std::result::Result<BalanceOf<T>, DispatchError> {
            // 获取kitty所属用户，判断kitty是否存在
            let owner = Self::owner(kitty_id).ok_or(Error::<T>::InvalidKittyIndex)?;

            // 市场上kitty挂的价钱，判断kitty是否在市场上挂单
            let kitty_price =
                Self::kitties_market(kitty_id).ok_or(Error::<T>::InvalidMarketPrice)?;

            // 出的钱要比市场上的价钱高
            ensure!(price >= kitty_price, Error::<T>::PriceTooLow);

            // 转钱，把钱直接转给在市场上挂单卖的人
            T::Currency::transfer(sender, &owner, kitty_price, ExistenceRequirement::KeepAlive)?;

            // 从市场上撤下来
            KittiesMarket::<T>::remove(kitty_id);

            // 变一下kitty所属关系
            Owner::<T>::insert(kitty_id, Some(sender.clone()));

            Ok(kitty_price)
        }
    }
}
