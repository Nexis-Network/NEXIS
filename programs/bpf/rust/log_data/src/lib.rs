//! Example Rust-based BPF program that uses nzt_log_data syscall

#![cfg(feature = "program")]

use nexis_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, log::nzt_log_data,
    program::set_return_data, pubkey::Pubkey,
};

entrypoint!(process_instruction);
#[allow(clippy::cognitive_complexity)]
fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let fields: Vec<&[u8]> = instruction_data.split(|e| *e == 0).collect();

    set_return_data(&[0x08, 0x01, 0x44]);

    nzt_log_data(&fields);

    Ok(())
}
