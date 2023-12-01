#![allow(clippy::integer_arithmetic)]
use {
    nexis_cli::cli::{process_command, CliCommand, CliConfig},
    nexis_client::rpc_client::RpcClient,
    nexis_faucet::faucet::run_local_faucet,
    nexis_sdk::{
        commitment_config::CommitmentConfig,
        native_token::nzt_to_lamports,
        signature::{Keypair, Signer},
    },
    nexis_streamer::socket::SocketAddrSpace,
    nexis_test_validator::TestValidator,
};

#[test]
fn test_cli_request_airdrop() {
    let mint_keypair = Keypair::new();
    let mint_pubkey = mint_keypair.pubkey();
    let faucet_addr = run_local_faucet(mint_keypair, None);
    let test_validator =
        TestValidator::with_no_fees(mint_pubkey, Some(faucet_addr), SocketAddrSpace::Unspecified);

    let mut bob_config = CliConfig::recent_for_tests();
    bob_config.json_rpc_url = test_validator.rpc_url();
    bob_config.command = CliCommand::Airdrop {
        pubkey: None,
        lamports: nzt_to_lamports(50.0),
    };
    let keypair = Keypair::new();
    bob_config.signers = vec![&keypair];

    let sig_response = process_command(&bob_config);
    sig_response.unwrap();

    let rpc_client =
        RpcClient::new_with_commitment(test_validator.rpc_url(), CommitmentConfig::processed());

    let balance = rpc_client
        .get_balance(&bob_config.signers[0].pubkey())
        .unwrap();
    assert_eq!(balance, nzt_to_lamports(50.0));
}
