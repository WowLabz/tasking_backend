use crate::mock::ExtBuilder;
use crate::{mock::*, Error, Status, TaskDetails, TaskTypeTags, CourtDispute, UserType, TaskStorage};
use frame_support::{log, assert_noop, assert_ok, dispatch::DispatchError};
use frame_system::{ensure_signed, RawOrigin};
use crate::AccountDetails;

#[test]
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
}

#[test]
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
        assert_ok!(Tasking::bid_for_task(
            Origin::signed(3),
            0,
            b"Bob".to_vec()
        ));
    })
}

#[test]
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
        assert_ok!(
            Tasking::task_completed(
                Origin::signed(3), 
                0, 
                vec![b"http://aws/worker.png".to_vec()]
            ));
    })
}

#[test]
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
        assert_ok!(Tasking::approve_task(Origin::signed(1), 0, 5));
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
            assert_ok!(
                Tasking::provide_customer_rating(
                Origin::signed(3),
                0,
                5
            ));
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
fn test_accept_jury_duty(){
    ExtBuilder::default()
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
        
        assert_ok!(
            Tasking::accept_jury_duty(
            Origin::signed(7),
            0)
        );
    })

}

        

