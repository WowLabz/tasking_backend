use crate::mock::ExtBuilder;
use crate::{mock::*, Error, Status, ProjectDetails, MilestoneHelper, TaskTypeTags, UserType};
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
        Tasking::create_project(
            Origin::signed(1),
            b"Alice".to_vec(),
            b"Project".to_vec(),
            vec![TaskTypeTags::WebDevelopment],
            Tasking::get_milestone_helper(),
            vec![]
        ).unwrap();
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
        Tasking::create_project(
            Origin::signed(1),
            b"Alice".to_vec(),
            b"Project".to_vec(),
            vec![TaskTypeTags::WebDevelopment],
            Tasking::get_milestone_helper(),
            vec![]
        ).unwrap();
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
        Tasking::create_project(
            Origin::signed(1),
            b"Alice".to_vec(),
            b"Project".to_vec(),
            vec![TaskTypeTags::WebDevelopment],
            Tasking::get_milestone_helper(),
            vec![]
        ).unwrap();
        Tasking::add_milestones_to_project(
            Origin::signed(1), 
        1, 
        vec![Tasking::get_milestone_helper()]
        ).unwrap();
        assert_ok!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                1
            )
        );
    });
}

#[test]
fn correct_error_for_adding_the_project_to_marketplace() {
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
        Tasking::create_project(
            Origin::signed(1),
            b"Alice".to_vec(),
            b"Project".to_vec(),
            vec![TaskTypeTags::WebDevelopment],
            Tasking::get_milestone_helper(),
            vec![]
        ).unwrap();
        assert_noop!(
            Tasking::add_project_to_marketplace(
                Origin::none(), 
                1
            ),
            DispatchError::BadOrigin
        );
        assert_noop!(
            Tasking::add_project_to_marketplace(
                Origin::signed(1), 
                2
            ),
            Error::<Test>::ProjectDoesNotExist
        );
        assert_noop!(
            Tasking::add_project_to_marketplace(
                Origin::signed(2), 
                2
            ),
            Error::<Test>::Unauthorised
        );
    });   
}