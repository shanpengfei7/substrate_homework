use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);
        // 创建一个kitty
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        // kitty的dna测试
        assert_eq!(Kitties::<Test>::get(1), Some(Kitty([215, 75, 66, 60, 234, 156, 146, 62, 247, 65, 230, 205, 192, 2, 31, 70])));
        // kitty属于用于1
        assert_eq!(Owner::<Test>::get(1), Some(1));
        // kitty数量是1
        assert_eq!(KittiesCount::<Test>::get(), Some(1));
    })
}

#[test]
fn create_failed_when_kitty_index_limit_over() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);
        KittiesCount::<Test>::put(u32::MAX);
        assert_noop!(KittiesModule::create(Origin::signed(1)), Error::<Test>::KittiesCountOverflow);
    })
}

#[test]
fn transfer_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);
        // 先创建一个kitty
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        // 转移
        assert_ok!(KittiesModule::transfer(Origin::signed(1), 2, 1));
        // kitty属于用于2
        assert_eq!(Owner::<Test>::get(1), Some(2));
        // kitty的dna不变
        assert_eq!(Kitties::<Test>::get(1), Some(Kitty([215, 75, 66, 60, 234, 156, 146, 62, 247, 65, 230, 205, 192, 2, 31, 70])));
        // 总数量还是1个
        assert_eq!(KittiesCount::<Test>::get(), Some(1));
    })
}

#[test]
fn transfer_failed_when_kitty_not_self() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);
        // 先创建一个kitty
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        // 用户2把用户1的kitty转给用户3时，NotOwner
        assert_noop!(KittiesModule::transfer(Origin::signed(2), 3, 1), Error::<Test>::NotOwner);
    })
}


#[test]
fn breed_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);
        // 用户1创建一个kitty1
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        // 用户2创建一个kitty2
        assert_ok!(KittiesModule::create(Origin::signed(2)));
        // 用户3用kitty1和kitty2生成kitty3
        assert_ok!(KittiesModule::breed(Origin::signed(3), 1, 2));
        // kitty属于用于3
        assert_eq!(Owner::<Test>::get(3), Some(3));
        // kitty的dna不变
        assert_eq!(Kitties::<Test>::get(3), Some(Kitty([221, 159, 35, 52, 178, 136, 42, 59, 222, 76, 238, 149, 192, 19, 55, 82])));
        // 总数量还是3个
        assert_eq!(KittiesCount::<Test>::get(), Some(3));
    })
}

#[test]
fn breed_failed_when_kitty1_equal_kitty2() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);
        // 用户1创建一个kitty1
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        // 用户3用kitty1和kitty1生成kitty3，SameParentIndex
        assert_noop!(KittiesModule::breed(Origin::signed(3), 1, 1), Error::<Test>::SameParentIndex);
    })
}

#[test]
fn breed_failed_when_kitty1_inalid() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);
        // 用户1创建一个kitty1
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        // 用户3用kitty1和kitty1生成kitty3，SameParentIndex
        assert_noop!(KittiesModule::breed(Origin::signed(3), 2, 1), Error::<Test>::InvalidKittyIndex);
    })
}

#[test]
fn breed_failed_when_kitty2_inalid() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);
        // 用户1创建一个kitty1
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        // 用户3用kitty1和kitty1生成kitty3，SameParentIndex
        assert_noop!(KittiesModule::breed(Origin::signed(3), 1, 2), Error::<Test>::InvalidKittyIndex);
    })
}


#[test]
fn breed_failed_when_kitty_index_limit_over() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);
        // 用户1创建一个kitty1
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        // 用户2创建一个kitty2
        assert_ok!(KittiesModule::create(Origin::signed(2)));

        KittiesCount::<Test>::put(u32::MAX);
        assert_noop!(KittiesModule::breed(Origin::signed(3), 1, 2), Error::<Test>::KittiesCountOverflow);
    })
}

