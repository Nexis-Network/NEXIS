#![allow(clippy::integer_arithmetic)]

/// This module contains the implementation of the Nexis Blockchain genesis process.

/// This module contains the implementation of the Nexis Blockchain genesis process.
pub mod address_generator;
pub mod nexis_genesis_accounts;
pub mod stakes;
pub mod unlocks;

use serde::{Deserialize, Serialize};

/// An account where the data is encoded as a Base64 string.
#[derive(Serialize, Deserialize, Debug)]
pub struct Base64Account {
    /// The balance of the account.
    pub balance: u64,
    /// The owner of the account.
    pub owner: String,
    /// The data encoded as a Base64 string.
    pub data: String,
    /// Indicates whether the account is executable.
    pub executable: bool,
}
