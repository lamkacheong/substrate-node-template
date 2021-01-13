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

		assert_eq!(<UserKitties<Test>>::iter_prefix_values(1).collect::<Vec<KittyIndex>>(),vec![0]);


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
//创建kitty,超过数量范围
fn create_kitty_exceed_maximun(){
	new_test_ext().execute_with(|| {
		run_to_block(10);

		//模拟kitty数量达到最大的情况
		KittiesCount::put(KittyIndex::max_value());
		assert_noop!(
			KittiesModule::create(Origin::signed(1)),
			Error::<Test>::KittiesCountOverflow
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
//正常breed
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

		//对user_kitties的测试
		//assert_eq!(UserKitties().iter_prefix_values(1).collect(),vec![0,1,2]);
		assert_eq!(<UserKitties<Test>>::iter_prefix_values(1).collect::<Vec<KittyIndex>>(),vec![0,2,1]);

		//对parent的测试
		assert_eq!(KittiesModule::parents(2,1), 1);
		assert_eq!(KittiesModule::parents(2,0), 0);

		assert_eq!(Parents::iter_prefix_values(2).collect::<Vec<KittyIndex>>(),vec![0,1]);
		//对children的测试
		assert_eq!(KittiesModule::children(0,2), 2);
		assert_eq!(KittiesModule::children(1,2), 2);
		assert_eq!(Children::iter_prefix_values(0).collect::<Vec<KittyIndex>>(),vec![2]);
		assert_eq!(Children::iter_prefix_values(1).collect::<Vec<KittyIndex>>(),vec![2]);

		//对breeded的测试
		assert_eq!(KittiesModule::breeded(0,1), 1);
		assert_eq!(KittiesModule::breeded(1,0), 0);

		assert_eq!(Breeded::iter_prefix_values(0).collect::<Vec<KittyIndex>>(),vec![1]);
		assert_eq!(Breeded::iter_prefix_values(1).collect::<Vec<KittyIndex>>(),vec![0]);

	})
}

#[test]
//breed的parent传入同一个id
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
