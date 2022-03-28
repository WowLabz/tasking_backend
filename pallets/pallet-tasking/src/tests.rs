use crate::mock::ExtBuilder;
use crate::{mock::*, Error, Status, AccountDetails, ProjectDetails, MilestoneHelper, TaskTypeTags, UserType};
use frame_support::{assert_noop, assert_ok, dispatch::DispatchError};
use frame_system::{ensure_signed};

#[test]
fn it_works_for_creating_a_project_with_correct_details(){
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
    //let milestone = Tasking::get_milestone_helper();
    .execute_with(||{
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
    });
}

#[test]
fn correct_error_for_creating_a_project_with_incorrect_details(){
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
        assert_noop!(
            Tasking::create_project(
                Origin::none(),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            ),
            DispatchError::BadOrigin
        );
    })
}

#[test]
fn it_works_for_adding_a_miletone_to_a_project_with_correct_details(){
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
    //let milestone = Tasking::get_milestone_helper();
    .execute_with(||{
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
                1, 
                vec![Tasking::get_milestone_helper()]
            )
        );
    });
}

#[test]
fn correct_error_for_adding_milestones_to_the_project(){
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
    //let milestone = Tasking::get_milestone_helper();
    .execute_with(||{
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_noop!(
            Tasking::add_milestones_to_project(
                Origin::none(), 
                1, 
                vec![Tasking::get_milestone_helper()]
            ),
            DispatchError::BadOrigin
        );
        assert_noop!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
                2, 
                vec![Tasking::get_milestone_helper()]
            ),
            Error::<Test>::ProjectDoesNotExist
        );
        assert_noop!(
            Tasking::add_milestones_to_project(
                Origin::signed(2), 
                1, 
                vec![Tasking::get_milestone_helper()]
            ),
            Error::<Test>::Unauthorised
        );
        assert_noop!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
                1, 
                vec![Tasking::get_milestone_helper(),Tasking::get_milestone_helper(),Tasking::get_milestone_helper(),Tasking::get_milestone_helper(),Tasking::get_milestone_helper(),Tasking::get_milestone_helper()]
            ),
            Error::<Test>::MilestoneLimitReached
        );
    });
}

#[test]
fn it_works_for_adding_the_project_to_marketplace_with_correct_details() {
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
            1, 
            vec![Tasking::get_milestone_helper()]
            )
        );
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
    });
}

#[test]
fn correct_error_for_adding_project_to_marketplace() {
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
            1, 
            vec![Tasking::get_milestone_helper()]
            )
        );
        assert_noop!(
            Tasking::add_project_to_marketplace(
                Origin::none(), 
                1
            ),
            DispatchError::BadOrigin
        );
        assert_noop!(
            Tasking::add_project_to_marketplace(
                Origin::signed(2),
                1
            ),
            Error::<Test>::Unauthorised
        );
        assert_noop!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1),
                2
            ),
            Error::<Test>::ProjectDoesNotExist
        );
    });
}


#[test]
fn it_works_for_bidding_for_a_milestone_with_correct_details() {
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
            1, 
            vec![Tasking::get_milestone_helper()]
            )
        );
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
        assert_ok!(
            Tasking::bid_for_milestone(
                Origin::signed(2), 
                b"1a".to_vec(), 
                b"Bob".to_vec()
            )
        );
    });
}

#[test]
fn correct_error_for_bidding_for_a_milestone() {
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
        assert_noop!(
            Tasking::bid_for_milestone(
                Origin::none(),
                b"1a".to_vec(),
                b"Bob".to_vec()
            ),
            DispatchError::BadOrigin
        );
        assert_noop!(
            Tasking::bid_for_milestone(
                Origin::signed(1),
                b"1a".to_vec(),
                b"Bob".to_vec()
            ),
            Error::<Test>::PublisherCannotBid
        );
        assert_noop!(
            Tasking::bid_for_milestone(
                Origin::signed(2),
                b"2a".to_vec(),
                b"Bob".to_vec()
            ),
            Error::<Test>::ProjectDoesNotExist
        );
        assert_noop!(
            Tasking::bid_for_milestone(
                Origin::signed(2),
                b"1b".to_vec(),
                b"Bob".to_vec()
            ),
            Error::<Test>::InvalidMilestoneId
        );
    });
}

#[test]
fn it_works_for_accepting_a_bid_for_a_milestone_with_correct_details() {
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
            1, 
            vec![Tasking::get_milestone_helper()]
            )
        );
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
        assert_ok!(
            Tasking::bid_for_milestone(
                Origin::signed(2), 
                b"1a".to_vec(), 
                b"Bob".to_vec()
            )
        );
        assert_ok!(
            Tasking::accept_bid(
                Origin::signed(1), 
                b"1a".to_vec(), 
                1
            )
        );
    });
}

#[test]
fn correct_error_while_accepting_a_bid() {
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
            1, 
            vec![Tasking::get_milestone_helper()]
            )
        );
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
        assert_ok!(
            Tasking::bid_for_milestone(
                Origin::signed(2), 
                b"1a".to_vec(), 
                b"Bob".to_vec()
            )
        );
        assert_noop!(
            Tasking::accept_bid(
                Origin::none(),
                b"1a".to_vec(),
                1
            ),
            DispatchError::BadOrigin
        );
        assert_noop!(
            Tasking::accept_bid(
                Origin::signed(2),
                b"1a".to_vec(),
                1
            ),
            Error::<Test>::Unauthorised
        );
        assert_noop!(
            Tasking::accept_bid(
                Origin::signed(1),
                b"2a".to_vec(),
                1
            ),
            Error::<Test>::ProjectDoesNotExist
        );
        assert_noop!(
            Tasking::accept_bid(
                Origin::signed(1),
                b"1c".to_vec(),
                1
            ),
            Error::<Test>::InvalidMilestoneId
        );
        assert_noop!(
            Tasking::accept_bid(
                Origin::signed(1),
                b"1a".to_vec(),
                2
            ),
            Error::<Test>::InvalidBidNumber
        );
    });
}

#[test]
fn it_works_for_completing_a_milestone_with_correct_details() {
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
            1, 
            vec![Tasking::get_milestone_helper()]
            )
        );
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
        assert_ok!(
            Tasking::bid_for_milestone(
                Origin::signed(2), 
                b"1a".to_vec(), 
                b"Bob".to_vec()
            )
        );
        assert_ok!(
            Tasking::accept_bid(
                Origin::signed(1), 
                b"1a".to_vec(), 
                1
            )
        );
        assert_ok!(
            Tasking::milestone_completed(
                Origin::signed(2),
                b"1a".to_vec(),
                vec![b"some attachment".to_vec()]
            )
        );
    });
}

#[test]
fn correct_error_for_completing_the_milestone() {
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
            1, 
            vec![Tasking::get_milestone_helper()]
            )
        );
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
        assert_ok!(
            Tasking::bid_for_milestone(
                Origin::signed(2), 
                b"1a".to_vec(), 
                b"Bob".to_vec()
            )
        );
        assert_ok!(
            Tasking::accept_bid(
                Origin::signed(1), 
                b"1a".to_vec(), 
                1
            )
        );
        assert_noop!(
            Tasking::milestone_completed(
                Origin::none(),
                b"1a".to_vec(),
                vec![b"some attachment".to_vec()]
            ),
            DispatchError::BadOrigin
        );
        assert_noop!(
            Tasking::milestone_completed(
                Origin::signed(2),
                b"2a".to_vec(),
                vec![b"some attachment".to_vec()]
            ),
            Error::<Test>::ProjectDoesNotExist
        );
        assert_noop!(
            Tasking::milestone_completed(
                Origin::signed(2),
                b"2a".to_vec(),
                vec![b"some attachment".to_vec()]
            ),
            Error::<Test>::ProjectDoesNotExist
        );
        assert_noop!(
            Tasking::milestone_completed(
                Origin::signed(1),
                b"1a".to_vec(),
                vec![b"some attachment".to_vec()]
            ),
            Error::<Test>::PublisherCannotCompleteMilestone
        );
        assert_noop!(
            Tasking::milestone_completed(
                Origin::signed(2),
                b"1c".to_vec(),
                vec![b"some attachment".to_vec()]
            ),
            Error::<Test>::InvalidMilestoneId
        );
        assert_noop!(
            Tasking::milestone_completed(
                Origin::signed(3),
                b"1a".to_vec(),
                vec![b"some attachment".to_vec()]
            ),
            Error::<Test>::UnauthorisedToComplete
        );
    });
}


#[test]
fn it_works_for_approving_a_milestone_with_correct_details() {
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
            1, 
            vec![Tasking::get_milestone_helper()]
            )
        );
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
        assert_ok!(
            Tasking::bid_for_milestone(
                Origin::signed(2), 
                b"1a".to_vec(), 
                b"Bob".to_vec()
            )
        );
        assert_ok!(
            Tasking::accept_bid(
                Origin::signed(1), 
                b"1a".to_vec(), 
                1
            )
        );
        assert_ok!(
            Tasking::milestone_completed(
                Origin::signed(2),
                b"1a".to_vec(),
                vec![b"some attachment".to_vec()]
            )
        );
        assert_ok!(
            Tasking::approve_milestone(
                Origin::signed(1),
                b"1a".to_vec(),
                4
            )
        );
    });
}

#[test]
fn correct_error_for_approving_a_milestone(){
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
            1, 
            vec![Tasking::get_milestone_helper()]
            )
        );
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
        assert_ok!(
            Tasking::bid_for_milestone(
                Origin::signed(2), 
                b"1a".to_vec(), 
                b"Bob".to_vec()
            )
        );
        assert_ok!(
            Tasking::accept_bid(
                Origin::signed(1), 
                b"1a".to_vec(), 
                1
            )
        );
        assert_ok!(
            Tasking::milestone_completed(
                Origin::signed(2),
                b"1a".to_vec(),
                vec![b"some attachment".to_vec()]
            )
        );
        assert_noop!(
            Tasking::approve_milestone(
                Origin::none(),
                b"1a".to_vec(),
                4
            ),
            DispatchError::BadOrigin
        );
        assert_noop!(
            Tasking::approve_milestone(
                Origin::signed(1),
                b"2a".to_vec(),
                4
            ),
            Error::<Test>::ProjectDoesNotExist
        );
        assert_noop!(
            Tasking::approve_milestone(
                Origin::signed(2),
                b"1a".to_vec(),
                4
            ),
            Error::<Test>::UnauthorisedToApprove
        );
        assert_noop!(
            Tasking::approve_milestone(
                Origin::signed(1),
                b"1d".to_vec(),
                4
            ),
            Error::<Test>::InvalidMilestoneId
        );
    });
}

#[test]
fn it_works_for_disapproving_a_milestone_with_correct_details() {
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
            1, 
            vec![Tasking::get_milestone_helper()]
            )
        );
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
        assert_ok!(
            Tasking::bid_for_milestone(
                Origin::signed(2), 
                b"1a".to_vec(), 
                b"Bob".to_vec()
            )
        );
        assert_ok!(
            Tasking::accept_bid(
                Origin::signed(1), 
                b"1a".to_vec(), 
                1
            )
        );
        assert_ok!(
            Tasking::milestone_completed(
                Origin::signed(2),
                b"1a".to_vec(),
                vec![b"some attachment".to_vec()]
            )
        );
        assert_ok!(
            Tasking::disapprove_milestone(
                Origin::signed(1),
                b"1a".to_vec()
            )
        );
    });
}

#[test]
fn correct_error_for_disapproving_a_milestone() {
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
            1, 
            vec![Tasking::get_milestone_helper()]
            )
        );
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
        assert_ok!(
            Tasking::bid_for_milestone(
                Origin::signed(2), 
                b"1a".to_vec(), 
                b"Bob".to_vec()
            )
        );
        assert_ok!(
            Tasking::accept_bid(
                Origin::signed(1), 
                b"1a".to_vec(), 
                1
            )
        );
        assert_ok!(
            Tasking::milestone_completed(
                Origin::signed(2),
                b"1a".to_vec(),
                vec![b"some attachment".to_vec()]
            )
        );
        assert_noop!(
            Tasking::disapprove_milestone(
                Origin::none(),
                b"1a".to_vec()
            ),
            DispatchError::BadOrigin
        );
        assert_noop!(
            Tasking::disapprove_milestone(
                Origin::signed(1),
                b"2a".to_vec()
            ),
            Error::<Test>::ProjectDoesNotExist
        );
        assert_noop!(
            Tasking::disapprove_milestone(
                Origin::signed(1),
                b"1d".to_vec()
            ),
            Error::<Test>::InvalidMilestoneId
        );
        assert_noop!(
            Tasking::disapprove_milestone(
                Origin::signed(1),
                b"1b".to_vec()
            ),
            Error::<Test>::MilestoneNotPendingApproval
        );
        assert_noop!(
            Tasking::disapprove_milestone(
                Origin::signed(2),
                b"1a".to_vec()
            ),
            Error::<Test>::Unauthorised
        );

    });
}

#[test]
fn it_works_for_providing_customer_rating_with_correct_details() {
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
            1, 
            vec![Tasking::get_milestone_helper()]
            )
        );
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
        assert_ok!(
            Tasking::bid_for_milestone(
                Origin::signed(2), 
                b"1a".to_vec(), 
                b"Bob".to_vec()
            )
        );
        assert_ok!(
            Tasking::accept_bid(
                Origin::signed(1), 
                b"1a".to_vec(), 
                1
            )
        );
        assert_ok!(
            Tasking::milestone_completed(
                Origin::signed(2),
                b"1a".to_vec(),
                vec![b"some attachment".to_vec()]
            )
        );
        assert_ok!(
            Tasking::approve_milestone(
                Origin::signed(1),
                b"1a".to_vec(),
                4
            )
        );
        assert_ok!(
            Tasking::provide_customer_rating(
                Origin::signed(2), 
                b"1a".to_vec(), 
                4
            )
        );
    });   
}

#[test]
fn correct_error_for_providing_customer_rating() {
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
            1, 
            vec![Tasking::get_milestone_helper()]
            )
        );
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
        assert_ok!(
            Tasking::bid_for_milestone(
                Origin::signed(2), 
                b"1a".to_vec(), 
                b"Bob".to_vec()
            )
        );
        assert_ok!(
            Tasking::accept_bid(
                Origin::signed(1), 
                b"1a".to_vec(), 
                1
            )
        );
        assert_ok!(
            Tasking::milestone_completed(
                Origin::signed(2),
                b"1a".to_vec(),
                vec![b"some attachment".to_vec()]
            )
        );
        assert_ok!(
            Tasking::approve_milestone(
                Origin::signed(1),
                b"1a".to_vec(),
                4
            )
        );
        assert_noop!(
            Tasking::provide_customer_rating(
                Origin::none(), 
                b"1a".to_vec(), 
                4
            ),
            DispatchError::BadOrigin
        );
        assert_noop!(
            Tasking::provide_customer_rating(
                Origin::signed(1), 
                b"1a".to_vec(), 
                4
            ),
            Error::<Test>::PublisherCannotRateSelf
        );
        assert_noop!(
            Tasking::provide_customer_rating(
                Origin::signed(1), 
                b"2a".to_vec(), 
                4
            ),
            Error::<Test>::ProjectDoesNotExist
        );
        assert_noop!(
            Tasking::provide_customer_rating(
                Origin::signed(2), 
                b"1c".to_vec(), 
                4
            ),
            Error::<Test>::InvalidMilestoneId
        );
        assert_noop!(
            Tasking::provide_customer_rating(
                Origin::signed(2), 
                b"1b".to_vec(), 
                4
            ),
            Error::<Test>::MilestoneNotPendingRating
        );
        assert_noop!(
            Tasking::provide_customer_rating(
                Origin::signed(3), 
                b"1a".to_vec(), 
                4
            ),
            Error::<Test>::UnauthorisedToProvideCustomerRating
        );
    });
}

#[test]
fn it_works_for_disapproving_the_rating_with_correct_details(){
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
            1, 
            vec![Tasking::get_milestone_helper()]
            )
        );
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
        assert_ok!(
            Tasking::bid_for_milestone(
                Origin::signed(2), 
                b"1a".to_vec(), 
                b"Bob".to_vec()
            )
        );
        assert_ok!(
            Tasking::accept_bid(
                Origin::signed(1), 
                b"1a".to_vec(), 
                1
            )
        );
        assert_ok!(
            Tasking::milestone_completed(
                Origin::signed(2),
                b"1a".to_vec(),
                vec![b"some attachment".to_vec()]
            )
        );
        assert_ok!(
            Tasking::approve_milestone(
                Origin::signed(1),
                b"1a".to_vec(),
                4
            )
        );
        assert_ok!(
            Tasking::provide_customer_rating(
                Origin::signed(2), 
                b"1a".to_vec(), 
                4
            )
        );
        assert_ok!(
            Tasking::disapprove_rating(
                Origin::signed(1),
                b"1a".to_vec(),
                UserType::Customer
            )
        );
    });
}

#[test]
fn correct_error_for_disapproving_rating() {
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
            1, 
            vec![Tasking::get_milestone_helper()]
            )
        );
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
        assert_ok!(
            Tasking::bid_for_milestone(
                Origin::signed(2), 
                b"1a".to_vec(), 
                b"Bob".to_vec()
            )
        );
        assert_ok!(
            Tasking::accept_bid(
                Origin::signed(1), 
                b"1a".to_vec(), 
                1
            )
        );
        assert_ok!(
            Tasking::milestone_completed(
                Origin::signed(2),
                b"1a".to_vec(),
                vec![b"some attachment".to_vec()]
            )
        );
        assert_ok!(
            Tasking::approve_milestone(
                Origin::signed(1),
                b"1a".to_vec(),
                4
            )
        );
        assert_ok!(
            Tasking::provide_customer_rating(
                Origin::signed(2), 
                b"1a".to_vec(), 
                4
            )
        );
        assert_noop!(
            Tasking::disapprove_rating(
                Origin::none(),
                b"1a".to_vec(),
                UserType::Customer
            ),
            DispatchError::BadOrigin
        );
        assert_noop!(
            Tasking::disapprove_rating(
                Origin::signed(1),
                b"2a".to_vec(),
                UserType::Customer
            ),
            Error::<Test>::ProjectDoesNotExist
        );
        assert_noop!(
            Tasking::disapprove_rating(
                Origin::signed(1),
                b"1c".to_vec(),
                UserType::Customer
            ),
            Error::<Test>::InvalidMilestoneId
        );
        assert_noop!(
            Tasking::disapprove_rating(
                Origin::signed(2),
                b"1a".to_vec(),
                UserType::Customer
            ),
            Error::<Test>::Unauthorised
        );
        assert_noop!(
            Tasking::disapprove_rating(
                Origin::signed(1),
                b"1b".to_vec(),
                UserType::Customer
            ),
            Error::<Test>::CustomerRatingNotProvided
        );
    });
}

#[test]
fn it_works_for_accepting_jury_duty_with_correct_details() {
    ExtBuilder::default()
    .with_balances(vec![
        (1, 100000),
        (2, 100000),
        (3, 100000),
        (4, 100000),
        (5, 100000),
        (6, 100000),
        (7, 100000),
    ]).
    with_account_details(vec![
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
			tags: [TaskTypeTags::WebDevelopment, TaskTypeTags::DeepLearning].to_vec(),
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
            1, 
            vec![Tasking::get_milestone_helper()]
            )
        );
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
        assert_ok!(
            Tasking::bid_for_milestone(
                Origin::signed(2), 
                b"1a".to_vec(), 
                b"Bob".to_vec()
            )
        );
        assert_ok!(
            Tasking::accept_bid(
                Origin::signed(1), 
                b"1a".to_vec(), 
                1
            )
        );
        assert_ok!(
            Tasking::milestone_completed(
                Origin::signed(2),
                b"1a".to_vec(),
                vec![b"some attachment".to_vec()]
            )
        );
        assert_ok!(
            Tasking::approve_milestone(
                Origin::signed(1),
                b"1a".to_vec(),
                4
            )
        );
        assert_ok!(
            Tasking::provide_customer_rating(
                Origin::signed(2), 
                b"1a".to_vec(), 
                4
            )
        );
        assert_ok!(
            Tasking::disapprove_rating(
                Origin::signed(1),
                b"1a".to_vec(),
                UserType::Customer
            )
        );
        assert_ok!(
            Tasking::accept_jury_duty(
                Origin::signed(3),
                b"1a".to_vec(),
                UserType::Customer,
                5,
                4
            )
        );
    });
}

#[test]
fn correct_error_for_accepting_jury_duty() {
    ExtBuilder::default()
    .with_balances(vec![
        (1, 100000),
        (2, 100000),
        (3, 100000),
        (4, 100000),
        (5, 100000),
        (6, 100000),
        (7, 100000),
    ]).
    with_account_details(vec![
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
			tags: [TaskTypeTags::WebDevelopment, TaskTypeTags::DeepLearning].to_vec(),
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
            1, 
            vec![Tasking::get_milestone_helper()]
            )
        );
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
        assert_ok!(
            Tasking::bid_for_milestone(
                Origin::signed(2), 
                b"1a".to_vec(), 
                b"Bob".to_vec()
            )
        );
        assert_ok!(
            Tasking::accept_bid(
                Origin::signed(1), 
                b"1a".to_vec(), 
                1
            )
        );
        assert_ok!(
            Tasking::milestone_completed(
                Origin::signed(2),
                b"1a".to_vec(),
                vec![b"some attachment".to_vec()]
            )
        );
        assert_ok!(
            Tasking::approve_milestone(
                Origin::signed(1),
                b"1a".to_vec(),
                4
            )
        );
        assert_ok!(
            Tasking::provide_customer_rating(
                Origin::signed(2), 
                b"1a".to_vec(), 
                4
            )
        );
        assert_ok!(
            Tasking::disapprove_rating(
                Origin::signed(1),
                b"1a".to_vec(),
                UserType::Customer
            )
        );
        assert_noop!(
            Tasking::accept_jury_duty(
                Origin::none(),
                b"1a".to_vec(),
                UserType::Customer,
                5,
                4
            ),
            DispatchError::BadOrigin
        );
        assert_noop!(
            Tasking::accept_jury_duty(
                Origin::signed(3),
                b"2a".to_vec(),
                UserType::Customer,
                5,
                4
            ),
            Error::<Test>::ProjectDoesNotExist
        );
        assert_noop!(
            Tasking::accept_jury_duty(
                Origin::signed(3),
                b"1c".to_vec(),
                UserType::Customer,
                5,
                4
            ),
            Error::<Test>::InvalidMilestoneId
        );
        assert_noop!(
            Tasking::accept_jury_duty(
                Origin::signed(1),
                b"1a".to_vec(),
                UserType::Customer,
                5,
                4
            ),
            Error::<Test>::NotPotentialJuror
        );
    });
}

#[test]
fn it_works_for_closing_the_milestone_with_correct_details() {
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
            1, 
            vec![Tasking::get_milestone_helper()]
            )
        );
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
        assert_ok!(
            Tasking::bid_for_milestone(
                Origin::signed(2), 
                b"1a".to_vec(), 
                b"Bob".to_vec()
            )
        );
        assert_ok!(
            Tasking::accept_bid(
                Origin::signed(1), 
                b"1a".to_vec(), 
                1
            )
        );
        assert_ok!(
            Tasking::milestone_completed(
                Origin::signed(2),
                b"1a".to_vec(),
                vec![b"some attachment".to_vec()]
            )
        );
        assert_ok!(
            Tasking::approve_milestone(
                Origin::signed(1),
                b"1a".to_vec(),
                4
            )
        );
        assert_ok!(
            Tasking::provide_customer_rating(
                Origin::signed(2), 
                b"1a".to_vec(), 
                4
            )
        );
        assert_ok!(
            Tasking::close_milestone(
                Origin::signed(1),
                b"1a".to_vec()
            )
        );
    });
}

#[test]
fn correct_error_while_closing_the_milestones() {
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
            1, 
            vec![Tasking::get_milestone_helper()]
            )
        );
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
        assert_ok!(
            Tasking::bid_for_milestone(
                Origin::signed(2), 
                b"1a".to_vec(), 
                b"Bob".to_vec()
            )
        );
        assert_ok!(
            Tasking::accept_bid(
                Origin::signed(1), 
                b"1a".to_vec(), 
                1
            )
        );
        assert_ok!(
            Tasking::milestone_completed(
                Origin::signed(2),
                b"1a".to_vec(),
                vec![b"some attachment".to_vec()]
            )
        );
        assert_ok!(
            Tasking::approve_milestone(
                Origin::signed(1),
                b"1a".to_vec(),
                4
            )
        );
        assert_ok!(
            Tasking::provide_customer_rating(
                Origin::signed(2), 
                b"1a".to_vec(), 
                4
            )
        );
        assert_noop!(
            Tasking::close_milestone(
                Origin::none(),
                b"1a".to_vec()
            ),
            DispatchError::BadOrigin
        );
        assert_noop!(
            Tasking::close_milestone(
                Origin::signed(1),
                b"2a".to_vec()
            ),
            Error::<Test>::ProjectDoesNotExist
        );
        assert_noop!(
            Tasking::close_milestone(
                Origin::signed(1),
                b"1c".to_vec()
            ),
            Error::<Test>::InvalidMilestoneId
        );
        assert_noop!(
            Tasking::close_milestone(
                Origin::signed(1),
                b"1b".to_vec()
            ),
            Error::<Test>::CustomerRatingNotProvided
        );
    });
}

#[test]
fn it_works_for_closing_the_project_with_correct_details() {
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
            1, 
            vec![Tasking::get_milestone_helper()]
            )
        );
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
        assert_ok!(
            Tasking::bid_for_milestone(
                Origin::signed(2), 
                b"1a".to_vec(), 
                b"Bob".to_vec()
            )
        );
        assert_ok!(
            Tasking::accept_bid(
                Origin::signed(1), 
                b"1a".to_vec(), 
                1
            )
        );
        assert_ok!(
            Tasking::milestone_completed(
                Origin::signed(2),
                b"1a".to_vec(),
                vec![b"some attachment".to_vec()]
            )
        );
        assert_ok!(
            Tasking::approve_milestone(
                Origin::signed(1),
                b"1a".to_vec(),
                4
            )
        );
        assert_ok!(
            Tasking::provide_customer_rating(
                Origin::signed(2), 
                b"1a".to_vec(), 
                4
            )
        );
        assert_ok!(
            Tasking::close_milestone(
                Origin::signed(1),
                b"1a".to_vec()
            )
        );
        assert_ok!(
            Tasking::close_project(
                Origin::signed(1),
                1
            )
        );
    });
}

fn correct_error_for_closing_the_project() {
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
        assert_ok!(
            Tasking::create_project(
                Origin::signed(1),
                b"Alice".to_vec(),
                b"Project".to_vec(),
                vec![TaskTypeTags::WebDevelopment],
                Tasking::get_milestone_helper(),
                vec![]
            )
        );
        assert_ok!(
            Tasking::add_milestones_to_project(
                Origin::signed(1), 
            1, 
            vec![Tasking::get_milestone_helper()]
            )
        );
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
        assert_ok!(
            Tasking::bid_for_milestone(
                Origin::signed(2), 
                b"1a".to_vec(), 
                b"Bob".to_vec()
            )
        );
        assert_ok!(
            Tasking::accept_bid(
                Origin::signed(1), 
                b"1a".to_vec(), 
                1
            )
        );
        assert_ok!(
            Tasking::milestone_completed(
                Origin::signed(2),
                b"1a".to_vec(),
                vec![b"some attachment".to_vec()]
            )
        );
        assert_ok!(
            Tasking::approve_milestone(
                Origin::signed(1),
                b"1a".to_vec(),
                4
            )
        );
        assert_ok!(
            Tasking::provide_customer_rating(
                Origin::signed(2), 
                b"1a".to_vec(), 
                4
            )
        );
        assert_ok!(
            Tasking::close_milestone(
                Origin::signed(1),
                b"1a".to_vec()
            )
        );
        assert_noop!(
            Tasking::close_project(
                Origin::none(),
                1
            ),
            DispatchError::BadOrigin
        );
        assert_noop!(
            Tasking::close_project(
                Origin::signed(2),
                1
            ),
            Error::<Test>::Unauthorised
        );
        assert_noop!(
            Tasking::close_project(
                Origin::signed(1),
                2
            ),
            Error::<Test>::ProjectDoesNotExist
        );
    });
}

