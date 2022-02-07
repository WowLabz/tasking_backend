use crate::mock::ExtBuilder;
use crate::{mock::*, Error, Status, TaskDetails, TaskTypeTags};
use frame_support::{assert_noop, assert_ok, dispatch::DispatchError};
use frame_system::{ensure_signed, RawOrigin};

#[test]
fn test_create_task (){
	new_test_ext().execute_with(|| {

	})

}

fn correct_error_for_unsigned_origin_while_creating_task_with_correct_() {
    new_test_ext().execute_with(|| {
        // Ensure the expected error is thrown when no value is present.
        assert_noop!(
            Tasking::create_task(
                Origin::none(),
                30,
                300,
                b"Create a website".to_vec(),
                b"Alice".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Some(vec![b"http://aws/publisher.png".to_vec()])
            ),
            DispatchError::BadOrigin,
        );
    });
}


