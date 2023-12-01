#![cfg_attr(RUSTC_WITH_SPECIALIZATION, feature(min_specialization))]
#![allow(clippy::integer_arithmetic)]

pub mod authorized_voters;
pub mod vote_instruction;
pub mod vote_state;
pub mod vote_transaction;

#[macro_use]
extern crate nexis_metrics;

#[macro_use]
extern crate nexis_frozen_abi_macro;

pub use nexis_sdk::vote::program::{check_id, id};


/// Amount of stake to be in majority = 10k
pub const MIN_STAKERS_TO_BE_MAJORITY: u64 = 10_000 * nexis_sdk::native_token::LAMPORTS_PER_NZT;

/// Number of stakers with lamports more than 10k, to start filtering = 19
pub const NUM_MAJOR_STAKERS_FOR_FILTERING: usize = 19;