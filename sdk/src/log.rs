#![cfg(feature = "program")]

pub use nexis_program::log::*;

#[macro_export]
#[deprecated(
    since = "1.4.3",
    note = "Please use `nexis_program::log::info` instead"
)]
macro_rules! info {
    ($msg:expr) => {
        $crate::log::nzt_log($msg)
    };
}
