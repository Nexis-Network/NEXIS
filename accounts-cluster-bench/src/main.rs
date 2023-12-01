/// This module contains the main entry point and utility functions for the accounts-cluster-bench program.
/// 
/// The `accounts-cluster-bench` program is used to benchmark the performance of creating and closing Nexis accounts in a cluster.
/// It utilizes the Nexis SDK and RPC client to interact with the Nexis network.
/// 
/// The `airdrop_lamports` function is responsible for airdropping a specified amount of lamports to an account if the starting balance is less than the desired balance.
/// 
/// The `SeedTracker` struct is used to keep track of the maximum number of created and closed accounts.
/// 
/// The `make_create_message` function generates a message for creating accounts with a specified number of instructions and balance.
/// 
/// The `make_close_message` function generates a message for closing accounts with a specified number of instructions and balance.
/// 
/// The `run_accounts_bench` function is the main entry point for the benchmarking program.
/// It takes various parameters such as the entrypoint address, faucet address, payer keypairs, iterations, batch size, and more.
/// It performs the benchmarking by creating and closing accounts in batches using the provided parameters.
/// 
/// Note: This code is part of a larger program and may require additional context to fully understand its functionality.
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::process::exit;
use std::time::Duration;
use clap::{App, Arg};
use nexis_client::rpc_client::RpcClient;
use nexis_sdk::commitment_config::CommitmentConfig;
use nexis_sdk::signature::{Keypair, Signer};
use nexis_sdk::system_instruction;
use nexis_sdk::transaction::Transaction;
use nexis_sdk::transport::Result as TransportResult;
use nexis_sdk::value::Value;
use nexis_sdk::pubkey::Pubkey;
/// Airdrops the specified amount of lamports to the specified account if the starting balance is less than the desired balance.
/// 
/// # Arguments
/// * `client` - The RPC client used to interact with the Nexis network.
/// * `faucet_addr` - The address of the faucet from which to request the airdrop.
/// * `id` - The keypair of the account to receive the airdrop.
/// * `desired_balance` - The desired balance of the account after the airdrop.
/// 
/// # Returns
/// * `true` if the airdrop is successful, `false` otherwise.
pub fn airdrop_lamports(
    client: &RpcClient,
    faucet_addr: &SocketAddr,
    id: &Keypair,
    desired_balance: u64,
) -> bool {
    // Implementation omitted for brevity
    // ...
    true
}
#![allow(clippy::integer_arithmetic)]
use {
    clap::{crate_description, crate_name, value_t, values_t_or_exit, App, Arg},
    log::*,
    rand::{thread_rng, Rng},
    rayon::prelude::*,
    nexis_account_decoder::parse_token::spl_token_pubkey,
    nexis_clap_utils::input_parsers::pubkey_of,
    nexis_client::{rpc_client::RpcClient, transaction_executor::TransactionExecutor},
    nexis_faucet::faucet::{request_airdrop_transaction, FAUCET_PORT},
    nexis_gossip::gossip_service::discover,
    nexis_runtime::inline_spl_token,
    nexis_sdk::{
        commitment_config::CommitmentConfig,
        instruction::{AccountMeta, Instruction},
        message::Message,
        pubkey::Pubkey,
        rpc_port::DEFAULT_RPC_PORT,
        signature::{read_keypair_file, Keypair, Signer},
        system_instruction, system_program,
        transaction::Transaction,
    },
    nexis_streamer::socket::SocketAddrSpace,
    nexis_transaction_status::parse_token::spl_token_instruction,
    std::{
        cmp::min,
        net::SocketAddr,
        process::exit,
        sync::{
            atomic::{AtomicU64, Ordering},
            Arc,
        },
        thread::sleep,
        time::{Duration, Instant},
    },
};

pub fn airdrop_lamports(
    client: &RpcClient,
    faucet_addr: &SocketAddr,
    id: &Keypair,
    desired_balance: u64,
) -> bool {
    let starting_balance = client.get_balance(&id.pubkey()).unwrap_or(0);
    info!("starting balance {}", starting_balance);

    if starting_balance < desired_balance {
        let airdrop_amount = desired_balance - starting_balance;
        info!(
            "Airdropping {:?} lamports from {} for {}",
            airdrop_amount,
            faucet_addr,
            id.pubkey(),
        );

        let blockhash = client.get_latest_blockhash().unwrap();
        match request_airdrop_transaction(faucet_addr, &id.pubkey(), airdrop_amount, blockhash) {
            Ok(transaction) => {
                let mut tries = 0;
                loop {
                    tries += 1;
                    let result = client.send_and_confirm_transaction(&transaction);

                    if result.is_ok() {
                        break;
                    }
                    if tries >= 5 {
                        panic!(
                            "Error requesting airdrop: to addr: {:?} amount: {} {:?}",
                            faucet_addr, airdrop_amount, result
                        )
                    }
                }
            }
            Err(err) => {
                panic!(
                    "Error requesting airdrop: {:?} to addr: {:?} amount: {}",
                    err, faucet_addr, airdrop_amount
                );
            }
        };

        let current_balance = client.get_balance(&id.pubkey()).unwrap_or_else(|e| {
            panic!("airdrop error {}", e);
        });
        info!("current balance {}...", current_balance);

        if current_balance - starting_balance != airdrop_amount {
            info!(
                "Airdrop failed? {} {} {} {}",
                id.pubkey(),
                current_balance,
                starting_balance,
                airdrop_amount,
            );
        }
    }
    true
}

struct SeedTracker {
    max_created: Arc<AtomicU64>,
    max_closed: Arc<AtomicU64>,
}

fn make_create_message(
    keypair: &Keypair,
    base_keypair: &Keypair,
    max_created_seed: Arc<AtomicU64>,
    num_instructions: usize,
    balance: u64,
    maybe_space: Option<u64>,
    mint: Option<Pubkey>,
) -> Message {
    let space = maybe_space.unwrap_or_else(|| thread_rng().gen_range(0, 1000));

    let instructions: Vec<_> = (0..num_instructions)
        .into_iter()
        .flat_map(|_| {
            let program_id = if mint.is_some() {
                inline_spl_token::id()
            } else {
                system_program::id()
            };
            let seed = max_created_seed.fetch_add(1, Ordering::Relaxed).to_string();
            let to_pubkey =
                Pubkey::create_with_seed(&base_keypair.pubkey(), &seed, &program_id).unwrap();
            let mut instructions = vec![system_instruction::create_account_with_seed(
                &keypair.pubkey(),
                &to_pubkey,
                &base_keypair.pubkey(),
                &seed,
                balance,
                space,
                &program_id,
            )];
            if let Some(mint_address) = mint {
                instructions.push(spl_token_instruction(
                    spl_token::instruction::initialize_account(
                        &spl_token::id(),
                        &spl_token_pubkey(&to_pubkey),
                        &spl_token_pubkey(&mint_address),
                        &spl_token_pubkey(&base_keypair.pubkey()),
                    )
                    .unwrap(),
                ));
            }

            instructions
        })
        .collect();

    Message::new(&instructions, Some(&keypair.pubkey()))
}

fn make_close_message(
    keypair: &Keypair,
    base_keypair: &Keypair,
    max_created: Arc<AtomicU64>,
    max_closed: Arc<AtomicU64>,
    num_instructions: usize,
    balance: u64,
    spl_token: bool,
) -> Message {
    let instructions: Vec<_> = (0..num_instructions)
        .into_iter()
        .filter_map(|_| {
            let program_id = if spl_token {
                inline_spl_token::id()
            } else {
                system_program::id()
            };
            let max_created_seed = max_created.load(Ordering::Relaxed);
            let max_closed_seed = max_closed.load(Ordering::Relaxed);
            if max_closed_seed >= max_created_seed {
                return None;
            }
            let seed = max_closed.fetch_add(1, Ordering::Relaxed).to_string();
            let address =
                Pubkey::create_with_seed(&base_keypair.pubkey(), &seed, &program_id).unwrap();
            if spl_token {
                Some(spl_token_instruction(
                    spl_token::instruction::close_account(
                        &spl_token::id(),
                        &spl_token_pubkey(&address),
                        &spl_token_pubkey(&keypair.pubkey()),
                        &spl_token_pubkey(&base_keypair.pubkey()),
                        &[],
                    )
                    .unwrap(),
                ))
            } else {
                Some(system_instruction::transfer_with_seed(
                    &address,
                    &base_keypair.pubkey(),
                    seed,
                    &program_id,
                    &keypair.pubkey(),
                    balance,
                ))
            }
        })
        .collect();

    Message::new(&instructions, Some(&keypair.pubkey()))
}

#[allow(clippy::too_many_arguments)]
fn run_accounts_bench(
    entrypoint_addr: SocketAddr,
    faucet_addr: SocketAddr,
    payer_keypairs: &[&Keypair],
    iterations: usize,
    maybe_space: Option<u64>,
    batch_size: usize,
    close_nth_batch: u64,
    maybe_lamports: Option<u64>,
    num_instructions: usize,
    mint: Option<Pubkey>,
    reclaim_accounts: bool,
) {
    assert!(num_instructions > 0);
    let client =
        RpcClient::new_socket_with_commitment(entrypoint_addr, CommitmentConfig::confirmed());

    info!("Targeting {}", entrypoint_addr);

    let mut latest_blockhash = Instant::now();
    let mut last_log = Instant::now();
    let mut count = 0;
    let mut blockhash = client.get_latest_blockhash().expect("blockhash");
    let mut tx_sent_count = 0;
    let mut total_accounts_created = 0;
    let mut total_accounts_closed = 0;
    let mut balances: Vec<_> = payer_keypairs
        .iter()
        .map(|keypair| client.get_balance(&keypair.pubkey()).unwrap_or(0))
        .collect();
    let mut last_balance = Instant::now();

    let default_max_lamports = 1000;
    let min_balance = maybe_lamports.unwrap_or_else(|| {
        let space = maybe_space.unwrap_or(default_max_lamports);
        client
            .get_minimum_balance_for_rent_exemption(space as usize)
            .expect("min balance")
    });

    let base_keypair = Keypair::new();
    let seed_tracker = SeedTracker {
        max_created: Arc::new(AtomicU64::default()),
        max_closed: Arc::new(AtomicU64::default()),
    };

    info!("Starting balance(s): {:?}", balances);

    let executor = TransactionExecutor::new(entrypoint_addr);

    // Create and close messages both require 2 signatures, fake a 2 signature message to calculate fees
    let mut message = Message::new(
        &[
            Instruction::new_with_bytes(
                Pubkey::new_unique(),
                &[],
                vec![AccountMeta::new(Pubkey::new_unique(), true)],
            ),
            Instruction::new_with_bytes(
                Pubkey::new_unique(),
                &[],
                vec![AccountMeta::new(Pubkey::new_unique(), true)],
            ),
        ],
        None,
    );

    loop {
        if latest_blockhash.elapsed().as_millis() > 10_000 {
            blockhash = client.get_latest_blockhash().expect("blockhash");
            latest_blockhash = Instant::now();
        }

        message.recent_blockhash = blockhash;
        let fee = client
            .get_fee_for_message(&message)
            .expect("get_fee_for_message");
        let lamports = min_balance + fee;

        for (i, balance) in balances.iter_mut().enumerate() {
            if *balance < lamports || last_balance.elapsed().as_millis() > 2000 {
                if let Ok(b) = client.get_balance(&payer_keypairs[i].pubkey()) {
                    *balance = b;
                }
                last_balance = Instant::now();
                if *balance < lamports * 2 {
                    info!(
                        "Balance {} is less than needed: {}, doing aidrop...",
                        balance, lamports
                    );
                    if !airdrop_lamports(
                        &client,
                        &faucet_addr,
                        payer_keypairs[i],
                        lamports * 100_000,
                    ) {
                        warn!("failed airdrop, exiting");
                        return;
                    }
                }
            }
        }

        // Create accounts
        let sigs_len = executor.num_outstanding();
        if sigs_len < batch_size {
            let num_to_create = batch_size - sigs_len;
            if num_to_create >= payer_keypairs.len() {
                info!("creating {} new", num_to_create);
                let chunk_size = num_to_create / payer_keypairs.len();
                if chunk_size > 0 {
                    for (i, keypair) in payer_keypairs.iter().enumerate() {
                        let txs: Vec<_> = (0..chunk_size)
                            .into_par_iter()
                            .map(|_| {
                                let message = make_create_message(
                                    keypair,
                                    &base_keypair,
                                    seed_tracker.max_created.clone(),
                                    num_instructions,
                                    min_balance,
                                    maybe_space,
                                    mint,
                                );
                                let signers: Vec<&Keypair> = vec![keypair, &base_keypair];
                                Transaction::new(&signers, message, blockhash)
                            })
                            .collect();
                        balances[i] = balances[i].saturating_sub(lamports * txs.len() as u64);
                        info!("txs: {}", txs.len());
                        let new_ids = executor.push_transactions(txs);
                        info!("ids: {}", new_ids.len());
                        tx_sent_count += new_ids.len();
                        total_accounts_created += num_instructions * new_ids.len();
                    }
                }
            }

            if close_nth_batch > 0 {
                let num_batches_to_close =
                    total_accounts_created as u64 / (close_nth_batch * batch_size as u64);
                let expected_closed = num_batches_to_close * batch_size as u64;
                let max_closed_seed = seed_tracker.max_closed.load(Ordering::Relaxed);
                // Close every account we've created with seed between max_closed_seed..expected_closed
                if max_closed_seed < expected_closed {
                    let txs: Vec<_> = (0..expected_closed - max_closed_seed)
                        .into_par_iter()
                        .map(|_| {
                            let message = make_close_message(
                                payer_keypairs[0],
                                &base_keypair,
                                seed_tracker.max_created.clone(),
                                seed_tracker.max_closed.clone(),
                                1,
                                min_balance,
                                mint.is_some(),
                            );
                            let signers: Vec<&Keypair> = vec![payer_keypairs[0], &base_keypair];
                            Transaction::new(&signers, message, blockhash)
                        })
                        .collect();
                    balances[0] = balances[0].saturating_sub(fee * txs.len() as u64);
                    info!("close txs: {}", txs.len());
                    let new_ids = executor.push_transactions(txs);
                    info!("close ids: {}", new_ids.len());
                    tx_sent_count += new_ids.len();
                    total_accounts_closed += new_ids.len() as u64;
                }
            }
        } else {
            let _ = executor.drain_cleared();
        }

        count += 1;
        if last_log.elapsed().as_millis() > 3000 || count >= iterations {
            info!(
                "total_accounts_created: {} total_accounts_closed: {} tx_sent_count: {} loop_count: {} balance(s): {:?}",
                total_accounts_created, total_accounts_closed, tx_sent_count, count, balances
            );
            last_log = Instant::now();
        }
        if iterations != 0 && count >= iterations {
            break;
        }
        if executor.num_outstanding() >= batch_size {
            sleep(Duration::from_millis(500));
        }
    }
    executor.close();

    if reclaim_accounts {
        let executor = TransactionExecutor::new(entrypoint_addr);
        loop {
            let max_closed_seed = seed_tracker.max_closed.load(Ordering::Relaxed);
            let max_created_seed = seed_tracker.max_created.load(Ordering::Relaxed);

            if latest_blockhash.elapsed().as_millis() > 10_000 {
                blockhash = client.get_latest_blockhash().expect("blockhash");
                latest_blockhash = Instant::now();
            }
            message.recent_blockhash = blockhash;
            let fee = client
                .get_fee_for_message(&message)
                .expect("get_fee_for_message");

            let sigs_len = executor.num_outstanding();
            if sigs_len < batch_size && max_closed_seed < max_created_seed {
                let num_to_close = min(
                    batch_size - sigs_len,
                    (max_created_seed - max_closed_seed) as usize,
                );
                if num_to_close >= payer_keypairs.len() {
                    info!("closing {} accounts", num_to_close);
                    let chunk_size = num_to_close / payer_keypairs.len();
                    info!("{:?} chunk_size", chunk_size);
                    if chunk_size > 0 {
                        for (i, keypair) in payer_keypairs.iter().enumerate() {
                            let txs: Vec<_> = (0..chunk_size)
                                .into_par_iter()
                                .filter_map(|_| {
                                    let message = make_close_message(
                                        keypair,
                                        &base_keypair,
                                        seed_tracker.max_created.clone(),
                                        seed_tracker.max_closed.clone(),
                                        num_instructions,
                                        min_balance,
                                        mint.is_some(),
                                    );
                                    if message.instructions.is_empty() {
                                        return None;
                                    }
                                    let signers: Vec<&Keypair> = vec![keypair, &base_keypair];
                                    Some(Transaction::new(&signers, message, blockhash))
                                })
                                .collect();
                            balances[i] = balances[i].saturating_sub(fee * txs.len() as u64);
                            info!("close txs: {}", txs.len());
                            let new_ids = executor.push_transactions(txs);
                            info!("close ids: {}", new_ids.len());
                            tx_sent_count += new_ids.len();
                            total_accounts_closed += (num_instructions * new_ids.len()) as u64;
                        }
                    }
                }
            } else {
                let _ = executor.drain_cleared();
            }
            count += 1;
            if last_log.elapsed().as_millis() > 3000 || max_closed_seed >= max_created_seed {
                info!(
                    "total_accounts_closed: {} tx_sent_count: {} loop_count: {} balance(s): {:?}",
                    total_accounts_closed, tx_sent_count, count, balances
                );
                last_log = Instant::now();
            }

            if max_closed_seed >= max_created_seed {
                break;
            }
            if executor.num_outstanding() >= batch_size {
                sleep(Duration::from_millis(500));
            }
        }
        executor.close();
    }
}

fn main() {
    nexis_logger::setup_with_default("nexis=info");
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(nexis_version::version!())
        .arg(
            Arg::with_name("entrypoint")
                .long("entrypoint")
                .takes_value(true)
                .value_name("HOST:PORT")
                .help("RPC entrypoint address. Usually <ip>:8899"),
        )
        .arg(
            Arg::with_name("faucet_addr")
                .long("faucet")
                .takes_value(true)
                .value_name("HOST:PORT")
                .help("Faucet entrypoint address. Usually <ip>:9900"),
        )
        .arg(
            Arg::with_name("space")
                .long("space")
                .takes_value(true)
                .value_name("BYTES")
                .help("Size of accounts to create"),


        const DEFAULT_RPC_PORT: u16 = 8001;
        const FAUCET_PORT: u16 = 9900;

        fn main() {
            let matches = App::new("Accounts Cluster Bench")
                .arg(
                    Arg::with_name("entrypoint")
                        .long("entrypoint")
                        .takes_value(true)
                        .value_name("ADDRESS")
                        .help("Entrypoint address"),
                )
                .arg(
                    Arg::with_name("faucet_addr")
                        .long("faucet-addr")
                        .takes_value(true)
                        .value_name("ADDRESS")
                        .help("Faucet address"),
                )
                .arg(
                    Arg::with_name("space")
                        .long("space")
                        .takes_value(true)
                        .value_name("BYTES")
                        .help("Space to allocate for each account"),
                )
                .arg(
                    Arg::with_name("lamports")
                        .long("lamports")
                        .takes_value(true)
                        .value_name("LAMPORTS")
                        .help("How many lamports to fund each account"),
                )
                .arg(
                    Arg::with_name("identity")
                        .long("identity")
                        .takes_value(true)
                        .multiple(true)
                        .value_name("FILE")
                        .help("Keypair file"),
                )
                .arg(
                    Arg::with_name("batch_size")
                        .long("batch-size")
                        .takes_value(true)
                        .value_name("BYTES")
                        .help("Number of transactions to send per batch"),
                )
                .arg(
                    Arg::with_name("close_nth_batch")
                        .long("close-frequency")
                        .takes_value(true)
                        .value_name("BYTES")
                        .help(
                            "Every `n` batches, create a batch of close transactions for
                            the earliest remaining batch of accounts created.
                            Note: Should be > 1 to avoid situations where the close \
                            transactions will be submitted before the corresponding \
                            create transactions have been confirmed",
                        ),
                )
                .arg(
                    Arg::with_name("num_instructions")
                        .long("num-instructions")
                        .takes_value(true)
                        .value_name("NUM")
                        .help("Number of accounts to create on each transaction"),
                )
                .arg(
                    Arg::with_name("iterations")
                        .long("iterations")
                        .takes_value(true)
                        .value_name("NUM")
                        .help("Number of iterations to make. 0 = unlimited iterations."),
                )
                .arg(
                    Arg::with_name("check_gossip")
                        .long("check-gossip")
                        .help("Just use entrypoint address directly"),
                )
                .arg(
                    Arg::with_name("mint")
                        .long("mint")
                        .takes_value(true)
                        .help("Mint address to initialize account"),
                )
                .arg(
                    Arg::with_name("reclaim_accounts")
                        .long("reclaim-accounts")
                        .takes_value(false)
                        .help("Reclaim accounts after session ends; incompatible with --iterations 0"),
                )
                .get_matches();

            let skip_gossip = !matches.is_present("check_gossip");

            let port = if skip_gossip { DEFAULT_RPC_PORT } else { 8001 };
            let entrypoint_addr = matches
                .value_of("entrypoint")
                .map(|addr| addr.parse().unwrap_or_else(|e| {
                    eprintln!("failed to parse entrypoint address: {}", e);
                    exit(1)
                }))
                .unwrap_or_else(|| SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port));

            let faucet_addr = matches
                .value_of("faucet_addr")
                .map(|addr| addr.parse().unwrap_or_else(|e| {
                    eprintln!("failed to parse faucet address: {}", e);
                    exit(1)
                }))
                .unwrap_or_else(|| SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), FAUCET_PORT));

            let space = matches
                .value_of("space")
                .and_then(|s| s.parse().ok());

            let lamports = matches
                .value_of("lamports")
                .and_then(|s| s.parse().ok());

            let batch_size = matches
                .value_of("batch_size")
                .and_then(|s| s.parse().ok())
                .unwrap_or(4);

            let close_nth_batch = matches
                .value_of("close_nth_batch")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);

            let iterations = matches
                .value_of("iterations")
                .and_then(|s| s.parse().ok())
                .unwrap_or(10);

            let num_instructions = matches
                .value_of("num_instructions")
                .and_then(|s| s.parse().ok())
                .unwrap_or(1);

            if num_instructions == 0 || num_instructions > 500 {
                eprintln!("bad num_instructions: {}", num_instructions);
                exit(1);
            }

            let mint = matches
                .value_of("mint")
                .map(|s| s.parse().unwrap_or_else(|e| {
                    eprintln!("failed to parse mint address: {}", e);
                    exit(1)
                }));

            let payer_keypairs: Vec<_> = matches
                .values_of("identity")
                .map(|values| {
                    values
                        .map(|keypair_string| {
                            Keypair::from_file(keypair_string)
                                .unwrap_or_else(|_| panic!("bad keypair {:?}", keypair_string))
                        })
                        .collect()
                })
                .unwrap_or_else(Vec::new);

            let rpc_addr = if !skip_gossip {
                let rpc_addr = discover_entrypoint(&entrypoint_addr)
                    .unwrap_or_else(|err| {
                        eprintln!("Failed to discover {} node: {:?}", entrypoint_addr, err);
                        exit(1);
                    });

                info!("Found cluster entry: {:?}", rpc_addr);
                rpc_addr
            } else {
                info!("Using {:?} as the RPC address", entrypoint_addr);
                entrypoint_addr
            };

            run_accounts_bench(
                rpc_addr,
                faucet_addr,
                &payer_keypairs,
                iterations,
                space,
                batch_size,
                close_nth_batch,
                lamports,
                num_instructions,
                mint,
                matches.is_present("reclaim_accounts"),
            );
        }

        fn discover_entrypoint(entrypoint_addr: &SocketAddr) -> TransportResult<SocketAddr> {
            let (gossip_nodes, _) = nexis_net_utils::discover(
                None, // keypair
                Some(entrypoint_addr),
                None,                    // num_nodes
                Duration::from_secs(60), // timeout
                None,                    // find_node_by_pubkey
                Some(entrypoint_addr),   // find_node_by_gossip_addr
                None,                    // my_gossip_addr
                0,                       // my_shred_version
                nexis_net_utils::SocketAddrSpace::Unspecified,
            )?;

            Ok(gossip_nodes[0].rpc)
        }

        fn run_accounts_bench(
            rpc_addr: SocketAddr,
            faucet_addr: SocketAddr,
            payer_keypairs: &[&Keypair],
            iterations: usize,
            space: Option<u64>,
            batch_size: usize,
            close_nth_batch: u64,
            lamports: Option<u64>,
            num_instructions: usize,
            mint: Option<Pubkey>,
            reclaim_accounts: bool,
        ) {
            // Implementation of run_accounts_bench function
            // ...

            // Placeholder code for demonstration purposes
            println!("Running accounts bench...");
        }
