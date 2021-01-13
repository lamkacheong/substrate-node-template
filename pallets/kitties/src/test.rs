use crate::{Error, mock::*};
use frame_support::{
	assert_ok, assert_noop,
	traits::{OnFinalize, OnInitialize},
};
use super::*;
use frame_system::Phase;
use frame_system::EventRecord;


pub type System = frame_system::Module<Test>;

fn run_to_block(n: u64) {
	while System::block_number() < n {
		KittiesModule::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		KittiesModule::on_initialize(System::block_number());
	}
}

#[test]
//正常创建kitty
fn create_kitty_works(){
	new_test_ext().execute_with(|| {
		run_to_block(10);

		assert_eq!(KittiesModule::kitties_count(), 0);

		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_eq!(KittiesModule::kitties_count(), 1);
		assert_eq!(KittiesModule::kitty_owner(0), Some(1));

		assert_eq!(
			System::events(),
			vec![EventRecord {
				phase: Phase::Initialization,
				event: TestEvent::kitties_event(RawEvent::KittyCreated(1, 0)),
				topics: vec![] }
			]
		);
	})
}

#[test]
//正常转移kitty
fn transfer_kitty_works() {
	new_test_ext().execute_with(|| {
		run_to_block(10);

		assert_eq!(KittiesModule::kitties_count(), 0);

		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_eq!(KittiesModule::kitties_count(), 1);

		assert_ok!(KittiesModule::transfer(Origin::signed(1), 2, 0));
		assert_eq!(KittiesModule::kitties_count(), 1);

		assert_eq!(KittiesModule::kitty_owner(0), Some(2));

		assert_eq!(
			System::events(),
			vec![
				EventRecord {
				phase: Phase::Initialization,
				event: TestEvent::kitties_event(RawEvent::KittyCreated(1, 0)),
				topics: vec![]
			},
				EventRecord {
					phase: Phase::Initialization,
					event: TestEvent::kitties_event(RawEvent::Transferred(1, 2, 0)),
					topics: vec![]
				 },
			]
		);
	})
}

#[test]
//转移kitty,kitty_id不存在
fn transfer_kitty_with_nonexist_kitty() {
	new_test_ext().execute_with(|| {
		run_to_block(10);
		assert_noop!(
			KittiesModule::transfer(Origin::signed(1), 2, 0),
			Error::<Test>::InvalidKittyId
		);
	})
}

#[test]
fn breed_works(){
	new_test_ext().execute_with(|| {
		run_to_block(10);
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::create(Origin::signed(1)));

		assert_ok!(
			KittiesModule::breed(Origin::signed(1),0,1)
		);

		assert_eq!(KittiesModule::kitties_count(), 3);
		assert_eq!(KittiesModule::kitty_owner(2), Some(1));
	})
}

#[test]
fn breed_with_same_parent(){
	new_test_ext().execute_with(|| {
		run_to_block(10);
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::create(Origin::signed(1)));

		assert_noop!(
			KittiesModule::breed(Origin::signed(1),0,0),
			Error::<Test>::RequireDifferentParent
		);
	})
}
