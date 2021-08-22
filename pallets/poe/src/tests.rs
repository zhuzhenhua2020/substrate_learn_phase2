use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

//create_claim tests
#[test]
fn  create_claim_success(){
    new_test_ext().execute_with(||{
           let claim:Vec<u8> = vec![0,1];
           assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));
           assert_eq!(
                  Proofs::<Test>::get(&claim),
                  (1,frame_system::Pallet::<Test>::block_number())
           );
    })
}
#[test]
fn create_claim_failed_when_claim_already_exsit(){
    new_test_ext().execute_with(||{
           let claim:Vec<u8> = vec![0,1];
           let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());
           assert_noop!(
                 PoeModule::create_claim(Origin::signed(1),claim.clone()),
                 Error::<Test>::ProofAlreadyClaimed
           );     
    })     
}
#[test]
fn create_claim_failed_when_proof_length_exceed(){
    new_test_ext().execute_with(||{
           let claim:Vec<u8> = vec![0; 300];
           assert_noop!(
                 PoeModule::create_claim(Origin::signed(1),claim.clone()),
                 Error::<Test>::ProofLengthExceeded
           );     
    })     
}

// revoke_claim  tests
#[test]
fn revoke_claim_success(){
    new_test_ext().execute_with(||{
           let claim:Vec<u8> = vec![0,1];
           let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());
           assert_ok!(PoeModule::revoke_claim(Origin::signed(1),claim.clone()));
           assert_eq!(Proofs::<Test>::get(&claim),(0,0));
    })     
}
#[test]
fn revoke_claim_failed_when_claim_is_not_exist(){
    new_test_ext().execute_with(||{
           let claim:Vec<u8> = vec![0,1];
           assert_noop!(
                  PoeModule::revoke_claim(Origin::signed(1),claim.clone()),
                  Error::<Test>::NoSuchProof
			); 
	})		
}
// transfer_claim tests
#[test]
fn transfer_claim_success(){
	new_test_ext().execute_with(||{
		let claim:Vec<u8> = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());
		assert_ok!(PoeModule::transfer_claim(Origin::signed(1),claim.clone(),2));
		assert_eq!(
			Proofs::<Test>::get(&claim),
			(2,frame_system::Pallet::<Test>::block_number())
	    );
 })
}
#[test]
fn transfer_claim_failed_when_claim_is_not_exist(){
	new_test_ext().execute_with(||{
		let claim:Vec<u8> = vec![0,1];
		assert_noop!(
			   PoeModule::transfer_claim(Origin::signed(1),claim.clone(),2),
			   Error::<Test>::NoSuchProof
		 ); 
    })		
}
#[test]
fn transfer_claim_failed_when_claim_is_not_owned_by_current_account(){
	new_test_ext().execute_with(||{
		let claim:Vec<u8> = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(3),claim.clone());
		assert_noop!(
			   PoeModule::transfer_claim(Origin::signed(1),claim.clone(),2),
			   Error::<Test>::NotProofOwner
		 ); 
    })
}