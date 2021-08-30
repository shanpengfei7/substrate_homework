#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use codec::{Decode, Encode};
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*,
                        traits::{Randomness, Currency, ExistenceRequirement}};
    use frame_system::pallet_prelude::*;
    use sp_io::hashing::blake2_128;
    use sp_runtime::{traits::{AtLeast32BitUnsigned, Member, Bounded, One}};

    #[derive(Encode, Decode, Debug, PartialEq)]
    pub struct Kitty(pub [u8; 16]);

    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
        type KittyIndex: Parameter + Member + AtLeast32BitUnsigned + Bounded + One + Default + Copy;
        type Currency: Currency<Self::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn kitties_count)]
    pub type KittiesCount<T: Config> = StorageValue<_, T::KittyIndex>;

    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<Kitty>, ValueQuery>;

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
        KittyMarket(T::AccountId, T::KittyIndex, Option<BalanceOf<T>>),
        KittyBuy(T::AccountId, T::KittyIndex, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        KittiesCountOverflow,
        NotOwner,
        SameParentIndex,
        InvalidKittyIndex,
        InvalidAccountId,
        InvalidMarketPrice,
        PriceTooLow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn create(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let kitty_id = Self::next_kitty_id()?;

            let dna = Self::random_value(&who);

            Self::add_one_kitty(who.clone(), kitty_id, dna);

            Self::deposit_event(Event::KittyCreate(who, kitty_id));

            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn transfer(
            origin: OriginFor<T>,
            new_owner: T::AccountId,
            kitty_id: T::KittyIndex,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(
                Some(who.clone()) == Owner::<T>::get(kitty_id),
                Error::<T>::NotOwner
            );

            Owner::<T>::insert(kitty_id, Some(new_owner.clone()));

            Self::deposit_event(Event::KittyTransfer(who, new_owner, kitty_id));

            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn breed(
            origin: OriginFor<T>,
            kitty_id_1: T::KittyIndex,
            kitty_id_2: T::KittyIndex,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameParentIndex);

            let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyIndex)?;
            let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyIndex)?;

            let kitty_id = Self::next_kitty_id()?;

            let dna_1 = kitty1.0;
            let dna_2 = kitty2.0;

            let selector = Self::random_value(&who);
            let mut new_dna = [0u8; 16];

            for i in 0..dna_1.len() {
                new_dna[i] = (selector[i] & dna_1[i]) | (!selector[i] & dna_2[i]);
            }

            Self::add_one_kitty(who.clone(), kitty_id, new_dna);

            Self::deposit_event(Event::KittyBreed(who, kitty_id));

            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn market(
            origin: OriginFor<T>,
            kitty_id: T::KittyIndex,
            price: Option<BalanceOf<T>>
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(
                Some(who.clone()) == Owner::<T>::get(kitty_id),
                Error::<T>::NotOwner
            );

            KittiesMarket::<T>::insert(kitty_id, price.clone());

            Self::deposit_event(Event::KittyMarket(who, kitty_id, price));

            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn buy(
            origin: OriginFor<T>,
            kitty_id: T::KittyIndex,
            price: BalanceOf<T>
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let owner = Self::owner(kitty_id).ok_or(Error::<T>::InvalidAccountId)?;

            let kitty_price = Self::kitties_market(kitty_id).ok_or(Error::<T>::InvalidMarketPrice)?;

            ensure!(price >= kitty_price, Error::<T>::PriceTooLow);

            T::Currency::transfer(&who, &owner, kitty_price, ExistenceRequirement::KeepAlive)?;

            KittiesMarket::<T>::remove(kitty_id);

            Owner::<T>::insert(kitty_id, Some(who.clone()));

            Self::deposit_event(Event::KittyBuy(who, kitty_id, kitty_price));

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn random_value(sender: &T::AccountId) -> [u8; 16] {
            let payload = (
                T::Randomness::random_seed(),
                &sender,
                <frame_system::Pallet<T>>::extrinsic_index(),
            );
            payload.using_encoded(blake2_128)
        }

        fn next_kitty_id() -> sp_std::result::Result<T::KittyIndex, DispatchError> {
            let kitty_id = match Self::kitties_count() {
                Some(id) => {
                    ensure!(
                        id != T::KittyIndex::max_value(),
                        Error::<T>::KittiesCountOverflow
                    );
                    id + T::KittyIndex::one()
                }
                None => T::KittyIndex::one(),
            };
            Ok(kitty_id)
        }

        fn add_one_kitty(owner: T::AccountId, kitty_id: T::KittyIndex, dna: [u8; 16]) {
            Kitties::<T>::insert(kitty_id, Some(Kitty(dna)));
            Owner::<T>::insert(kitty_id, Some(owner.clone()));
            KittiesCount::<T>::put(kitty_id);
        }
    }
}
