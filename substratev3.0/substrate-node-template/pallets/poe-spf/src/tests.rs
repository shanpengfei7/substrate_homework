use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

// 测试创建存证
#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        // 测试创建存证
        assert_ok!(PoeSpfModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((1, frame_system::Pallet::<Test>::block_number()))
        );
    })
}

// 当存证长度超过限制时创建存在，返回ClaimLimitOver
// 因为在mock中配置的长度是4，所以测试时传入的存证长度是5
#[test]
fn create_claim_failed_when_claim_limit_over() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1, 2, 3, 4];
        // 先创建一个存证
        assert_noop!(
            PoeSpfModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimLimitOver
        );
    })
}

// 当存证已存在时，再次创建同一存证返回ProofAlreadyExist
#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        // 先创建一个存证
        let _ = PoeSpfModule::create_claim(Origin::signed(1), claim.clone());
        // 创建一个重复的存证
        assert_noop!(
            PoeSpfModule::create_claim(Origin::signed(1), claim),
            Error::<Test>::ProofAlreadyExist
        );
    })
}

// 测试删除存证
#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        // 先创建一个存证
        let _ = PoeSpfModule::create_claim(Origin::signed(1), claim.clone());

        // 删除存证
        assert_ok!(PoeSpfModule::revoke_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), None);
    })
}

// 当存证不存在时删除存证，返回ClaimNotExist
#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        // 测试删除一个不存在的存证
        let claim = vec![0, 1];
        assert_noop!(
            PoeSpfModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    })
}

// 当存证不存在时删除存证，返回ClaimNotExist
#[test]
fn revoke_claim_failed_when_claim_is_not_oneself() {
    new_test_ext().execute_with(|| {
        // 测试删除一个不存在的存证
        let claim = vec![0, 1];
        // 先创建一个存证
        let _ = PoeSpfModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeSpfModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    })
}

// 测试存证转移
#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        // 先创建一个存证
        assert_ok!(PoeSpfModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((1, frame_system::Pallet::<Test>::block_number()))
        );
        // 转移存证，判断存证确实已转移
        assert_ok!(PoeSpfModule::transfer_claim(
            Origin::signed(1),
            claim.clone(),
            9
        ));
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((9, frame_system::Pallet::<Test>::block_number()))
        );
    })
}

// 当存证已被转移后，二次转移返回NotClaimOwner
#[test]
fn transfer_claim_when_already_transfer() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        // 先创建一个存证
        assert_ok!(PoeSpfModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((1, frame_system::Pallet::<Test>::block_number()))
        );
        // 第一次转移
        let _ = PoeSpfModule::transfer_claim(Origin::signed(1), claim.clone(), 9);
        // 第二次转移
        assert_noop!(
            PoeSpfModule::transfer_claim(Origin::signed(1), claim, 9),
            Error::<Test>::NotClaimOwner
        );
    })
}

// 当存证转移给自己时返回OwnerEqualReceiver
#[test]
fn transfer_claim_when_2_address_equal() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        // 先创建一个存证
        assert_ok!(PoeSpfModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((1, frame_system::Pallet::<Test>::block_number()))
        );
        // 测试转移给自己
        assert_noop!(
            PoeSpfModule::transfer_claim(Origin::signed(1), claim, 1),
            Error::<Test>::OwnerEqualReceiver
        );
    })
}
