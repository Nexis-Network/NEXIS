pub mod cli;
pub mod exit_code;
pub mod extensions;
pub mod ledger;
pub mod routines;
pub mod timestamp;

use clap::Parser;
use cli::{Cli, Commands};

lazy_static::lazy_static! {
    pub static ref IS_EMBED: bool = std::env::args().any(|arg| &arg == "--embed");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dotenv = dotenvy::dotenv();

    match std::env::var("RUST_LOG") {
        Ok(value) => {
            env_logger::init();
            log::info!(r#"RUST_LOG is set to "{value}""#);
        }
        Err(_err) => {
            std::env::set_var("RUST_LOG", "evm_block_recovery");
            env_logger::init();
            log::warn!(r#"Environment variable "RUST_LOG" not found."#);
        }
    }

    match dotenv {
        Ok(path) => {
            log::info!(r#""{}" successfully loaded"#, path.display())
        }
        Err(e) => {
            log::warn!(r#"".env" file not found: {e:?}""#)
        }
    }

    let cli = Cli::parse();
    match cli.command {
        Commands::FindEvm {
            start_block,
            end_block,
            limit,
            bigtable_limit,
        } => {
            if end_block.is_none() && limit.is_none() {
                log::error!("Not enough arguments to calculate `end_slot`");
                exit_with_code(Err(exit_code::INVALID_ARGUMENTS))
            }
            let end_block = calculate_end_block(start_block, end_block, limit);
            let result = routines::find_evm(
                cli.creds,
                cli.instance,
                start_block,
                end_block,
                bigtable_limit,
            )
            .await;

            exit_with_code(result)
        }
        Commands::FindNative {
            start_block,
            end_block,
            limit,
            bigtable_limit,
        } => {
            if end_block.is_none() && limit.is_none() {
                log::error!("Not enough arguments to calculate `end_slot`");
                exit_with_code(Err(exit_code::INVALID_ARGUMENTS))
            }
            let end_block = calculate_end_block(start_block, end_block, limit);
            let result = routines::find_native(
                cli.creds,
                cli.instance,
                start_block,
                end_block,
                bigtable_limit,
            )
            .await;
            exit_with_code(result)
        }
        Commands::RestoreChain {
            first_block,
            last_block,
            archive_url,
            modify_ledger,
            force_resume,
            timestamps,
            output_dir,
        } => {
            routines::restore_chain(
                ledger::with_params(cli.creds, cli.instance).await?,
                first_block,
                last_block,
                archive_url,
                modify_ledger,
                force_resume,
                timestamps,
                output_dir,
            )
            .await
        }
        Commands::CheckNative { slot } => {
            routines::check_native(ledger::with_params(cli.creds, cli.instance).await?, slot).await
        }
        Commands::CheckEvm { block_number } => {
            routines::check_evm(
                ledger::with_params(cli.creds, cli.instance).await?,
                block_number,
            )
            .await
        }
        Commands::CompareNative {
            start_slot,
            limit,
            credible_ledger_creds,
            credible_ledger_instance,
            dubious_ledger_creds,
            dubious_ledger_instance,
        } => {
            routines::compare_native(
                start_slot,
                limit,
                ledger::with_params(Some(credible_ledger_creds), credible_ledger_instance).await?,
                ledger::with_params(Some(dubious_ledger_creds), dubious_ledger_instance).await?,
            )
            .await
        }
        Commands::Upload { collection } => {
            routines::upload(
                ledger::with_params(cli.creds, cli.instance).await?,
                collection,
            )
            .await
        }
        Commands::RepeatEvm {
            block_number,
            limit,
            src_creds,
            src_instance,
            dst_creds,
            dst_instance,
        } => {
            routines::repeat_evm(
                block_number,
                limit,
                ledger::with_params(Some(src_creds), src_instance).await?,
                ledger::with_params(Some(dst_creds), dst_instance).await?,
            )
            .await
        }
        Commands::RepeatNative {
            start_slot,
            end_slot,
            src_creds,
            src_instance,
            dst_creds,
            dst_instance,
        } => {
            routines::repeat_native(
                start_slot,
                end_slot,
                ledger::with_params(Some(src_creds), src_instance).await?,
                ledger::with_params(Some(dst_creds), dst_instance).await?,
            )
            .await
        }
    }
}

fn calculate_end_block(start_block: u64, end_block: Option<u64>, limit: Option<u64>) -> u64 {
    if let Some(end_block) = end_block {
        return end_block;
    }

    if let Some(limit) = limit {
        return start_block + limit - 1;
    }

    unreachable!()
}

fn exit_with_code(result: std::result::Result<(), i32>) -> ! {
    match result {
        Ok(_) => std::process::exit(0),
        Err(code) => std::process::exit(code),
    }
}
