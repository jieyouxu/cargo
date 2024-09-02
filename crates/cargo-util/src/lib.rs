//! Miscellaneous support code used by Cargo.

#![allow(clippy::disallowed_methods)]

pub use self::read2::read2;
pub use du::du;
pub use process_builder::ProcessBuilder;
pub use process_error::{exit_status_to_string, is_simple_exit_code, ProcessError};
pub use sha256::Sha256;

mod du;
pub mod paths;
mod process_builder;
mod process_error;
mod read2;
pub mod registry;
mod sha256;

#[cfg(windows)]
mod windows_hacks;

/// Whether or not this running in a Continuous Integration environment.
pub fn is_ci() -> bool {
    std::env::var("CI").is_ok() || std::env::var("TF_BUILD").is_ok()
}
