//use super::*;
use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn test_should_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(
            Chat::write_message(Origin::signed(1), 2, b"Hello there!".to_vec())
        );
    })
}