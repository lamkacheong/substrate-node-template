use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

#[test]
fn create_claim_works(){
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0, 1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

		assert_eq!(Proofs::<Test>::get(&claim), (1, frame_system::Module::<Test>::block_number()));
	})
}

#[test]
fn create_claim_failed_when_claim_already_exit(){
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}

#[test]
fn revoke_clain_works(){
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_ok!(
			PoeModule::revoke_claim(Origin::signed(1), claim.clone())
		);
	})
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(||{
		let claim: Vec<u8> = vec![0, 1];
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	})
}

#[test]
fn revoke_claim_failed_when_claim_is_not_owned() {
	new_test_ext().execute_with(||{
		let claim: Vec<u8> = vec![0, 1];

		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}

#[test]
fn transfer_works(){
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));

		assert_eq!(Proofs::<Test>::get(&claim), (2, frame_system::Module::<Test>::block_number()));
	})
}

#[test]
fn transfer_fails_when_claim_not_exist(){
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0, 1];
		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2),
			Error::<Test>::ClaimNotExist
		);
	})
}

#[test]
fn transfer_tails_when_claim_not_owned(){
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 2),
			Error::<Test>::NotClaimOwner
		);
	})
}


#[test]
fn check_long_proof_works(){
	new_test_ext().execute_with(|| {
		let mut claim:Vec<u8> = vec![];
		let mut i = 0;
		while i < 260 {
			i += 1;
			claim.push(0);
		}
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofTooLong
		);
	})
}
