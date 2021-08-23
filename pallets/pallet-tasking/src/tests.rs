use crate::{mock::*, Error, Status, TaskDetails};
use frame_support::{assert_noop, assert_ok, dispatch::DispatchError};
use frame_system::ensure_signed;

#[test]
fn correct_error_for_unsigned_origin_while_creating_task_with_correct_() {
    new_test_ext().execute_with(|| {
        // Ensure the expected error is thrown when no value is present.
        assert_noop!(
            PalletTasking::create_task(Origin::none(), 30, 300, b"Create a website".to_vec()),
            DispatchError::BadOrigin,
        );
    });
}

#[test]
fn it_works_for_creating_a_task_with_correct_details_provided() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(PalletTasking::create_task(
            Origin::signed(1),
            30,
            300,
            b"Create a website".to_vec()
        ));
        // Read pallet storage and assert an expected result.
        let sender = ensure_signed(Origin::signed(1)).unwrap();
        let expected_task_details = TaskDetails {
            task_id: 0,
            publisher: sender.clone(),
            worker_id: None,
            task_deadline: 30,
            cost: 300,
            status: Status::Open,
            task_description: b"Create a website".to_vec(),
        };
        assert_eq!(PalletTasking::task(0), expected_task_details);
    });
}

#[test]
fn correct_error_for_bidding_a_task_with_incorrect_task_id() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            PalletTasking::bid_for_task(Origin::signed(3), 10),
            Error::<Test>::TaskDoesNotExist
        );
    })
}

#[test]
fn correct_error_for_bidding_a_task_with_the_same_publisher_id() {
    new_test_ext().execute_with(|| {
        PalletTasking::create_task(Origin::signed(1), 50, 500, b"Backend Systems".to_vec())
            .unwrap();
        assert_noop!(
            PalletTasking::bid_for_task(Origin::signed(1), 0),
            Error::<Test>::UnauthorisedToBid
        );
    })
}

#[test]
fn correct_error_for_bidding_a_task_with_incorrect_status() {
    new_test_ext().execute_with(|| {
        PalletTasking::create_task(Origin::signed(1), 50, 500, b"Backend Systems".to_vec())
            .unwrap();
        PalletTasking::bid_for_task(Origin::signed(3), 0).unwrap();
        PalletTasking::task_completed(Origin::signed(3), 0).unwrap();
        assert_noop!(
            PalletTasking::bid_for_task(Origin::signed(3), 0),
            Error::<Test>::TaskIsNotOpen
        );
    })
}

#[test]
fn it_works_for_bidding_a_task_with_correct_details() {
    new_test_ext().execute_with(|| {
        PalletTasking::create_task(Origin::signed(1), 50, 500, b"Backend Systems".to_vec())
            .unwrap();
        assert_ok!(PalletTasking::bid_for_task(Origin::signed(3), 0));
    })
}

#[test]
fn correct_error_for_completing_a_task_with_incorrect_task_id() {
    new_test_ext().execute_with(|| {
        PalletTasking::create_task(Origin::signed(1), 50, 500, b"Backend Systems".to_vec())
            .unwrap();
        PalletTasking::bid_for_task(Origin::signed(3), 0).unwrap();
        assert_noop!(
            PalletTasking::task_completed(Origin::signed(3), 10),
            Error::<Test>::TaskDoesNotExist
        );
    })
}

#[test]
fn correct_error_for_completing_a_task_with_same_publisher_id() {
    new_test_ext().execute_with(|| {
        PalletTasking::create_task(Origin::signed(1), 50, 500, b"Backend Systems".to_vec())
            .unwrap();
        PalletTasking::bid_for_task(Origin::signed(3), 0).unwrap();
        assert_noop!(
            PalletTasking::task_completed(Origin::signed(1), 0),
            Error::<Test>::UnauthorisedToComplete
        );
    })
}

#[test]
fn correct_error_for_completing_a_task_with_incorrect_status() {
    new_test_ext().execute_with(|| {
        PalletTasking::create_task(Origin::signed(1), 50, 500, b"Backend Systems".to_vec())
            .unwrap();
        assert_noop!(
            PalletTasking::task_completed(Origin::signed(3), 0),
            Error::<Test>::TaskIsNotInProgress
        );
    })
}

#[test]
fn it_works_for_completing_a_task_with_correct_details() {
	new_test_ext().execute_with(|| {
        PalletTasking::create_task(Origin::signed(1), 50, 500, b"Backend Systems".to_vec())
            .unwrap();
        PalletTasking::bid_for_task(Origin::signed(3), 0).unwrap();
        assert_ok!(
            PalletTasking::task_completed(Origin::signed(3), 0)
        );
	})
}
