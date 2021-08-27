use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        // assert_eq!(Kitties::<Test>::get(&claim), Some(1));
        // assert_eq!(Owner::<Test>::get(&claim), Some(1));
        assert_eq!(KittiesCount::<Test>::get(), Some(1));
    })
}

#[test]
fn create_failed_when_claim_limit_over() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            KittiesModule::create(Origin::signed(1)),
            Error::<Test>::KittiesCountOverflow
        );
    })
}
