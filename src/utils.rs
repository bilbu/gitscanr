use crate::constants;
use std::process::exit;

/// Prints `message` to stderr and exits the process returning a [`FAILURE`] code.
///
/// [`FAILURE`]: crate::constants::exit::SUCCESS
///
/// # Example
///
/// ```
/// mod utils;
///
/// utils::bail("Something bad happened.");
/// ```
pub fn bail(message: &str) {
    eprintln!("{}", message);
    exit(constants::exit::FAILURE); // TODO: decorate process::exit to encompass constants::exit::*
}

pub fn warn(message: &str) {
    eprintln!("{}", message);
}
