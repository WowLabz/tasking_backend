use crate::mock::ExtBuilder;
use crate::{mock::*, Error, Status, TaskDetails, TaskTypeTags, UserType};
use frame_support::{assert_noop, assert_ok, dispatch::DispatchError};
use frame_system::{ensure_signed};

#[test]
// Test functions for create_task extrinsic
fn test_create_task (){
    ExtBuilder::default()
    .with_balances(vec![
        (1, 100000),
        (2, 100000),
        (3, 100000),
        (4, 100000),
        (5, 100000),
        (6, 100000),
        (7, 100000),
    ])
    .build()

    //  Test for checking storage structure on creating a task
    .execute_with(|| {
        assert_ok!(Tasking::create_task(
            Origin::signed(1),
            30,
            300,
            b"Create a website".to_vec(),
            b"Alice".to_vec(),
            vec![TaskTypeTags::WebDevelopment],
            Some(vec![b"http://aws/publisher.png".to_vec()])
        ));
        // Read pallet storage and assert an expected result.
        let sender = ensure_signed(Origin::signed(1)).unwrap();
        let expected_task_details = TaskDetails {
            task_id: 0,
            publisher: sender.clone(),
            worker_id: None,
            publisher_name: Some(b"Alice".to_vec()),
            worker_name: None,
            task_tags: vec![TaskTypeTags::WebDevelopment],
            task_deadline: 30,
            cost: 300,
            status: Status::Open,
            task_description: b"Create a website".to_vec(),
            publisher_attachments: Some(vec![b"http://aws/publisher.png".to_vec()]),
            worker_attachments: None,
            dispute: None,
            final_worker_rating: None,
            final_customer_rating: None
        };
        assert_eq!(Tasking::task(0), expected_task_details);
    });

    // Test for chhecking error on an unsigned transaction
    new_test_ext().execute_with(||{
        assert_noop!(Tasking::create_task(
            Origin::none(),
            30,
            300,
            b"Create a website".to_vec(),
            b"Alice".to_vec(),
            vec![TaskTypeTags::WebDevelopment],
            Some(vec![b"http://aws/publisher.png".to_vec()])
        ),
        DispatchError::BadOrigin
        );
    })
}

#[test]
// Test for bid for task extrinsic
fn test_bid_for_task(){
    ExtBuilder::default()
        .with_balances(vec![
            (1, 100000),
            (2, 100000),
            (3, 100000),
            (4, 100000),
            (5, 100000),
            (6, 100000),
            (7, 100000),
        ])
        .build()
        
        .execute_with(||{
        Tasking::create_task(
            Origin::signed(1),
            50,
            500,
            b"Backend Systems".to_vec(),
            b"Alice".to_vec(),
            vec![TaskTypeTags::FullStackDevelopment], 
            Some(vec![b"http://aws/publisher.png".to_vec()])
        )
        .unwrap();

        // Test for bidding for a created task by a signed user
        assert_ok!(
            Tasking::bid_for_task(
                Origin::signed(3),
                0,
                b"Bob".to_vec()
            ));

        // Test for error return whe bidding a task with incorrect task ID
        assert_noop!(
            Tasking::bid_for_task(
                Origin::signed(3), 
                10, 
                b"Bob".to_vec()),
                Error::<Test>::TaskDoesNotExist
            );
    });

}

#[test]
// Test for completing a task extrinsic
fn test_task_completed() {
    ExtBuilder::default()
    .with_balances(vec![
        (1, 100000),
        (2, 100000),
        (3, 100000),
        (4, 100000),
        (5, 100000),
        (6, 100000),
        (7, 100000),
    ])
    .build()
    .execute_with(|| {
        Tasking::create_task(
            Origin::signed(1),
            50,
            500,
            b"Backend Systems".to_vec(),
            b"Alice".to_vec(),
            vec![TaskTypeTags::FullStackDevelopment], 
            Some(vec![b"http://aws/publisher.png".to_vec()])
        )
        .unwrap();
        Tasking::bid_for_task(
            Origin::signed(3), 
            0, 
            b"Bob".to_vec()
        ).unwrap();

        // Task being completed by the right worker
        assert_ok!(
            Tasking::task_completed(
                Origin::signed(3), 
                0, 
                vec![b"http://aws/worker.png".to_vec()]
            ));
        
        // Error test for trying to complete the task with a wrong ID
        assert_noop!(
            Tasking::task_completed(
                Origin::signed(4), 
                0, 
                vec![b"http://aws/worker.png".to_vec()]
            ),
            Error::<Test>::TaskIsNotInProgress
        );
    });
}

#[test]
// Tests for approve tasks extrinsic
fn test_approve_task() {
    ExtBuilder::default()
    .with_balances(vec![
        (1, 100000),
        (2, 100000),
        (3, 100000),
        (4, 100000),
        (5, 100000),
        (6, 100000),
        (7, 100000),
    ])
    .build()
    .execute_with(|| {
        Tasking::create_task(
            Origin::signed(1),
            50,
            500,
            b"Backend Systems".to_vec(),
            b"Alice".to_vec(),
            vec![TaskTypeTags::FullStackDevelopment],
            Some(vec![b"http://aws/publisher.png".to_vec()]),
        )
        .unwrap();
        Tasking::bid_for_task(Origin::signed(3), 0, b"Bob".to_vec()).unwrap();
        Tasking::task_completed(
            Origin::signed(3),
            0,
            vec![b"http://aws/worker.png".to_vec()],
        )
        .unwrap();
        
        // Success test for approving task
        assert_ok!(Tasking::approve_task(Origin::signed(1), 0, 5));

        // Error test for approving task with an account id other than the publishers
        assert_noop!(Tasking::approve_task(Origin::signed(2), 0, 5), Error::<Test>::TaskIsNotPendingApproval);
    })
}

#[test]
fn test_provide_customer_ratings(){
    ExtBuilder::default()
        .with_balances(vec![
            (1, 100000),
            (2, 100000),
            (3, 100000),
            (4, 100000),
            (5, 100000),
            (6, 100000),
            (7, 100000),
        ]) 
        .build()
        .execute_with(|| {
            Tasking::create_task(
                Origin::signed(1),
                50,
                500,
                b"Backend Systems".to_vec(),
                b"Alice".to_vec(),
                vec![TaskTypeTags::FullStackDevelopment],
                Some(vec![b"http://aws/publisher.png".to_vec()]),
            )
            .unwrap();
            Tasking::bid_for_task(
                Origin::signed(3), 
                0, 
                b"Bob".to_vec()
            ).unwrap();
            Tasking::task_completed(
                Origin::signed(3),
                0,
                vec![b"http://aws/worker.png".to_vec()],
            )
            .unwrap();
            Tasking::approve_task(
                Origin::signed(1), 
                0, 
                5
            ).unwrap();

            // Success test for providing customer rating
            assert_ok!(
                Tasking::provide_customer_rating(
                Origin::signed(3),
                0,
                5
            ));

            // Error test for providing rating to incorrect task ID
            assert_noop!(
                Tasking::provide_customer_rating(
                Origin::signed(3),
                2,
                5
            ), 
            Error::<Test>::TaskDoesNotExist
        );
        });
}

#[test]
fn test_close_task(){
    ExtBuilder::default()
        .with_balances(vec![
            (1, 100000),
            (2, 100000),
            (3, 100000),
            (4, 100000),
            (5, 100000),
            (6, 100000),
            (7, 100000),
        ])
        .build()
        .execute_with(|| {
            Tasking::create_task(
                Origin::signed(1),
                50,
                500,
                b"Backend Systems".to_vec(),
                b"Alice".to_vec(),
                vec![TaskTypeTags::FullStackDevelopment],
                Some(vec![b"http://aws/publisher.png".to_vec()]),
            )
            .unwrap();
            Tasking::bid_for_task(
                Origin::signed(3), 
                0, 
                b"Bob".to_vec()
            ).unwrap();
            Tasking::task_completed(
                Origin::signed(3),
                0,
                vec![b"http://aws/worker.png".to_vec()],
            )
            .unwrap();
            Tasking::approve_task(
                Origin::signed(1), 
                0, 
                5
            ).unwrap();
            Tasking::provide_customer_rating(
                Origin::signed(3),
                0,
                5
            ).unwrap();
            assert_ok!(
                Tasking::close_task(
                    Origin::signed(1),
                    0
                )
            );
        })
}

#[test]
fn test_disapprove_task(){
    ExtBuilder::default()
    .with_balances(vec![
        (1, 100000),
        (2, 100000),
        (3, 100000),
        (4, 100000),
        (5, 100000),
        (6, 100000),
        (7, 100000),
    ])
    .build()
    .execute_with(|| {
        Tasking::create_task(
            Origin::signed(1),
            50,
            500,
            b"Backend Systems".to_vec(),
            b"Alice".to_vec(),
            vec![TaskTypeTags::FullStackDevelopment],
            Some(vec![b"http://aws/publisher.png".to_vec()]),
        )
        .unwrap();
        Tasking::bid_for_task(
            Origin::signed(3), 
            0, 
            b"Bob".to_vec()
        ).unwrap();
        Tasking::task_completed(
            Origin::signed(3),
            0,
            vec![b"http://aws/worker.png".to_vec()],
        )
        .unwrap();
        assert_ok!(
            Tasking::disapprove_task(
                Origin::signed(1), 
                0)
            );
    })

}

#[test]
fn test_worker_disapprove_rating(){
    ExtBuilder::default()
    .with_balances(vec![
        (1, 100000),
        (2, 100000),
        (3, 100000),
        (4, 100000),
        (5, 100000),
        (6, 100000),
        (7, 100000),
    ])
    .build()
    .execute_with(|| {
        Tasking::create_task(
            Origin::signed(1),
            50,
            500,
            b"Backend Systems".to_vec(),
            b"Alice".to_vec(),
            vec![TaskTypeTags::FullStackDevelopment],
            Some(vec![b"http://aws/publisher.png".to_vec()]),
        )
        .unwrap();
        Tasking::bid_for_task(
            Origin::signed(3), 
            0, 
            b"Bob".to_vec()
        ).unwrap();
        Tasking::task_completed(
            Origin::signed(3),
            0,
            vec![b"http://aws/worker.png".to_vec()],
        )
        .unwrap();
        Tasking::approve_task(
            Origin::signed(1), 
            0, 
            5
        ).unwrap();
        assert_ok!(
            Tasking::disapprove_rating(
                Origin::signed(3),
                0,
                UserType::Worker
            )
        );
    })
    
}

#[test]
fn test_customer_disapprove_rating(){
    ExtBuilder::default()
    .with_balances(vec![
        (1, 100000),
        (2, 100000),
        (3, 100000),
        (4, 100000),
        (5, 100000),
        (6, 100000),
        (7, 100000),
    ])
    .build()
    .execute_with(|| {
        Tasking::create_task(
            Origin::signed(1),
            50,
            500,
            b"Backend Systems".to_vec(),
            b"Alice".to_vec(),
            vec![TaskTypeTags::FullStackDevelopment],
            Some(vec![b"http://aws/publisher.png".to_vec()]),
        )
        .unwrap();
        Tasking::bid_for_task(
            Origin::signed(3), 
            0, 
            b"Bob".to_vec()
        ).unwrap();
        Tasking::task_completed(
            Origin::signed(3),
            0,
            vec![b"http://aws/worker.png".to_vec()],
        )
        .unwrap();
        Tasking::approve_task(
            Origin::signed(1), 
            0, 
            5
        ).unwrap();
        Tasking::provide_customer_rating(
            Origin::signed(3),
            0,
            5
        ).unwrap();
        assert_ok!(
            Tasking::disapprove_rating(
                Origin::signed(1),
                0,
                UserType::Customer
            )
        );
    })
    
}

#[test]
fn raise_dispute(){
    ExtBuilder::default()
    .with_balances(vec![
        (1, 100000),
        (2, 100000),
        (3, 100000),
        (4, 100000),
        (5, 100000),
        (6, 100000),
        (7, 100000),
    ])
    .build()
    .execute_with(|| {
        Tasking::create_task(
            Origin::signed(1),
            50,
            500,
            b"Backend Systems".to_vec(),
            b"Alice".to_vec(),
            vec![TaskTypeTags::FullStackDevelopment],
            Some(vec![b"http://aws/publisher.png".to_vec()]),
        )
        .unwrap();
        Tasking::bid_for_task(
            Origin::signed(3), 
            0, 
            b"Bob".to_vec()
        ).unwrap();
        Tasking::task_completed(
            Origin::signed(3),
            0,
            vec![b"http://aws/worker.png".to_vec()],
        )
        .unwrap();
        Tasking::approve_task(
            Origin::signed(1), 
            0, 
            5
        ).unwrap();
        Tasking::provide_customer_rating(
            Origin::signed(3),
            0,
            5
        ).unwrap();
        assert_ok!(
            Tasking::raise_dispute(
                Origin::signed(3),
                0,
                UserType::Worker
            )
        );
    })

}

#[test]
fn test_accept_jury_duty(){
    ExtBuilder::default()
    .with_balances(vec![
        (1, 100000),
        (2, 100000),
        (3, 100000),
        (4, 100000),
        (5, 100000),
        (6, 100000),
        (7, 100000),
    ])
    .with_account_details(vec![
        (1, AccountDetails {
			balance: 1 << 60,
			ratings: [5, 5, 5, 5, 5].to_vec(),
			avg_rating: Some(5),
			tags: [TaskTypeTags::MachineLearning, TaskTypeTags::DeepLearning].to_vec(),
			sudo: true
		}),
        (2, AccountDetails {
			balance: 1 << 60,
			ratings: [5, 5, 5, 5, 5].to_vec(),
			avg_rating: Some(5),
			tags: [TaskTypeTags::MachineLearning, TaskTypeTags::DeepLearning].to_vec(),
			sudo: false
		}),
        (3, AccountDetails {
			balance: 1 << 60,
			ratings: [5, 5, 5, 5, 5].to_vec(),
			avg_rating: Some(5),
			tags: [TaskTypeTags::MachineLearning, TaskTypeTags::DeepLearning].to_vec(),
			sudo: false
		}),
        (4, AccountDetails {
			balance: 1 << 60,
			ratings: [5, 5, 5, 5, 5].to_vec(),
			avg_rating: Some(5),
			tags: [TaskTypeTags::MachineLearning, TaskTypeTags::DeepLearning].to_vec(),
			sudo: false
		}),
        (5, AccountDetails {
			balance: 1 << 60,
			ratings: [5, 5, 5, 5, 5].to_vec(),
			avg_rating: Some(5),
			tags: [TaskTypeTags::MachineLearning, TaskTypeTags::DeepLearning].to_vec(),
			sudo: false
		}),
        (6, AccountDetails {
			balance: 1 << 60,
			ratings: [5, 5, 5, 5, 5].to_vec(),
			avg_rating: Some(5),
			tags: [TaskTypeTags::MachineLearning, TaskTypeTags::DeepLearning].to_vec(),
			sudo: false
		}),
        (7, AccountDetails {
			balance: 1 << 60,
			ratings: [5, 5, 5, 5, 5].to_vec(),
			avg_rating: Some(5),
			tags: [TaskTypeTags::MachineLearning, TaskTypeTags::DeepLearning].to_vec(),
			sudo: false
		}),
    ])
    .build()
    .execute_with(|| {
        Tasking::create_task(
            Origin::signed(1),
            50,
            500,
            b"Backend Systems".to_vec(),
            b"Alice".to_vec(),
            vec![TaskTypeTags::MachineLearning],
            Some(vec![b"http://aws/publisher.png".to_vec()]),
        )
        .unwrap();
        Tasking::bid_for_task(
            Origin::signed(3), 
            0, 
            b"Bob".to_vec()
        ).unwrap();
        Tasking::task_completed(
            Origin::signed(3),
            0,
            vec![b"http://aws/worker.png".to_vec()],
        ).unwrap();
        Tasking::disapprove_task(
            Origin::signed(1), 
            0
        ).unwrap();
        System::set_block_number(6);
        Tasking::collect_cases(6);
        System::set_block_number(11);
        Tasking::collect_cases(11);
        // System::set_block_number(16);
        // Tasking::collect_cases(16);
        
        assert_ok!(
            Tasking::accept_jury_duty(
            Origin::signed(7),
            0)
        );
    })

}

#[test]
fn test_cast_vote(){
    ExtBuilder::default()
    .with_balances(vec![
        (1, 100000),
        (2, 100000),
        (3, 100000),
        (4, 100000),
        (5, 100000),
        (6, 100000),
        (7, 100000),
    ])
    .with_account_details(vec![
        (1, AccountDetails {
			balance: 1 << 60,
			ratings: [5, 5, 5, 5, 5].to_vec(),
			avg_rating: Some(5),
			tags: [TaskTypeTags::MachineLearning, TaskTypeTags::DeepLearning].to_vec(),
			sudo: true
		}),
        (2, AccountDetails {
			balance: 1 << 60,
			ratings: [5, 5, 5, 5, 5].to_vec(),
			avg_rating: Some(5),
			tags: [TaskTypeTags::MachineLearning, TaskTypeTags::DeepLearning].to_vec(),
			sudo: false
		}),
        (3, AccountDetails {
			balance: 1 << 60,
			ratings: [5, 5, 5, 5, 5].to_vec(),
			avg_rating: Some(5),
			tags: [TaskTypeTags::MachineLearning, TaskTypeTags::DeepLearning].to_vec(),
			sudo: false
		}),
        (4, AccountDetails {
			balance: 1 << 60,
			ratings: [5, 5, 5, 5, 5].to_vec(),
			avg_rating: Some(5),
			tags: [TaskTypeTags::MachineLearning, TaskTypeTags::DeepLearning].to_vec(),
			sudo: false
		}),
        (5, AccountDetails {
			balance: 1 << 60,
			ratings: [5, 5, 5, 5, 5].to_vec(),
			avg_rating: Some(5),
			tags: [TaskTypeTags::MachineLearning, TaskTypeTags::DeepLearning].to_vec(),
			sudo: false
		}),
        (6, AccountDetails {
			balance: 1 << 60,
			ratings: [5, 5, 5, 5, 5].to_vec(),
			avg_rating: Some(5),
			tags: [TaskTypeTags::MachineLearning, TaskTypeTags::DeepLearning].to_vec(),
			sudo: false
		}),
        (7, AccountDetails {
			balance: 1 << 60,
			ratings: [5, 5, 5, 5, 5].to_vec(),
			avg_rating: Some(5),
			tags: [TaskTypeTags::MachineLearning, TaskTypeTags::DeepLearning].to_vec(),
			sudo: false
		}),
    ])
    .build()
    .execute_with( ||{
        Tasking::create_task(
            Origin::signed(1),
            50,
            500,
            b"Backend Systems".to_vec(),
            b"Alice".to_vec(),
            vec![TaskTypeTags::MachineLearning],
            Some(vec![b"http://aws/publisher.png".to_vec()]),
        )
        .unwrap();
        //System::set_block_number(2);
        Tasking::bid_for_task(
            Origin::signed(3), 
            0, 
            b"Bob".to_vec()
        ).unwrap();
        //System::set_block_number(3);
        Tasking::task_completed(
            Origin::signed(3),
            0,
            vec![b"http://aws/worker.png".to_vec()],
        ).unwrap();
        System::set_block_number(3);
        Tasking::disapprove_task(
            Origin::signed(1), 
            0
        ).unwrap();
        
        Tasking::accept_jury_duty(
            Origin::signed(7),
            0
        ).unwrap();
        // Number shoud be equal block number at disapprove task + 5
        System::set_block_number(8);
        Tasking::collect_cases(8);
        
        System::set_block_number(10);
        let task = Tasking::task(0);
        println!("***Task Details: {:?}",task.status);
        assert_ok!(
            Tasking::cast_vote(
                Origin::signed(7),
                0,
                UserType::Worker,
                4,
                5
            )
        );

                
    })
}



        

