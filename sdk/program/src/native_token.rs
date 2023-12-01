#![allow(clippy::integer_arithmetic)]
/// There are 10^9 lamports in one NZT
pub const LAMPORTS_PER_NZT: u64 = 1_000_000_000;

/// Approximately convert fractional native tokens (lamports) into native tokens (NZT)
pub fn lamports_to_nzt(lamports: u64) -> f64 {
    lamports as f64 / LAMPORTS_PER_NZT as f64
}

/// Approximately convert native tokens (NZT) into fractional native tokens (lamports)
pub fn nzt_to_lamports(sol: f64) -> u64 {
    (sol * LAMPORTS_PER_NZT as f64) as u64
}

use std::fmt::{Debug, Display, Formatter, Result};
pub struct Sol(pub u64);

impl Sol {
    fn write_in_nzt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "◎{}.{:09}",
            self.0 / LAMPORTS_PER_NZT,
            self.0 % LAMPORTS_PER_NZT
        )
    }
}

impl Display for Sol {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_in_nzt(f)
    }
}

impl Debug for Sol {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_in_nzt(f)
    }
}
