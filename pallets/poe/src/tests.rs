use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0 as u8, 1 as u8];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(
            Proofs::<Test>::get(&claim),
            (1, frame_system::Pallet::<Test>::block_number())
        );
    })
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0 as u8, 1 as u8];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyClaimed
        );
    })
}

#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0 as u8, 1 as u8];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        let val = Proofs::<Test>::get(&claim);
        println!("{:?}", val);
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
        let val = Proofs::<Test>::get(&claim);
        println!("{:?}", val);
        assert_eq!(val, (0, 0));
    });
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0 as u8, 1 as u8];
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::NoSuchProof
        );
    });
}

#[test]
fn revoke_claim_failed_when_claim_not_have_permissions() {
    new_test_ext().execute_with(|| {
        let claim = vec![0 as u8, 1 as u8];
        // 创建凭证
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        // 使用其他用户撤销凭证
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotProofOwner
        );
    });
}

#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0 as u8, 1 as u8];
        // 创建凭证
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        // 转移凭证
        assert_ok!(PoeModule::transfer_claim(
            Origin::signed(1),
            claim.clone(),
            2
        ));
        // 查看凭证拥有者
        let val = Proofs::<Test>::get(&claim);
        assert_eq!(val, (2, 0));
    });
}

#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0 as u8, 1 as u8];
        assert_noop!(
            // 不创建凭证的前提下转移凭证
            PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 3),
            Error::<Test>::NoSuchProof
        );
    });
}

#[test]
fn transfer_claim_failed_when_claim_not_have_permissions() {
    new_test_ext().execute_with(|| {
        let claim = vec![0 as u8, 1 as u8];
        // 创建凭证
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(
            // 使用非拥有者转移凭证
            PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 3),
            Error::<Test>::NotProofOwner
        );
    });
}

#[test]
fn create_claim_failed_when_claim_is_too_long() {
    new_test_ext().execute_with(|| {
        // 一个长度超过限制的凭证
        let claim = vec![
            0 as u8, 1 as u8, 2 as u8, 3 as u8, 4 as u8, 5 as u8, 6 as u8,
        ];
        // 创建凭证时检查是否报错
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofCross
        );
    })
}
