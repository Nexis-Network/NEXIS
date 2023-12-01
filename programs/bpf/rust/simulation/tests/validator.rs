#![cfg(feature = "test-bpf")]

use {
    nexis_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
    nexis_sdk::{signature::Signer, transaction::Transaction},
    exzo_validator::test_validator::*,
};

#[test]
fn no_panic() {
    nexis_logger::setup_with_default("nexis_program_runtime=debug");
    let program_id = Pubkey::new_unique();

    let (test_validator, payer) = TestValidatorGenesis::default()
        .add_program("nexis_bpf_rust_simulation", program_id)
        .start();
    let rpc_client = test_validator.get_rpc_client();
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new_readonly(sysvar::slot_history::id(), false),
                AccountMeta::new_readonly(sysvar::clock::id(), false),
            ],
            data: vec![],
        }],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );

    rpc_client
        .send_and_confirm_transaction(&transaction)
        .unwrap();
}
