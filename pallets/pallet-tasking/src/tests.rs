use frame_system::ensure_signed;
use crate::{mock::*, Error, TaskDetails, Status};
use frame_support::{assert_noop, assert_ok, dispatch::DispatchError};


#[test]
fn throws_error_for_unsigned_origin_while_creating_task_with_correct_() {
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

