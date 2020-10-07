// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
#![forbid(bare_trait_objects)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
// Safety-critical application lints
#![deny(
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used
)]
#![allow(
    clippy::iter_nth_zero,
    clippy::match_bool,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]
// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing
// license files and more
//#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
//#![deny(clippy::missing_errors_doc, warnings)]

mod consts;
mod error;
pub use error::{Error, Result};
use std::{
    fs::File,
    io::Read
};
use std::path::Path;
use std::fmt::Debug;

#[cfg(test)]
mod unit_tests;

pub fn read_n_bytes<P: AsRef<Path> + Debug>(filename: P, n: usize) -> Result<Vec<u8>> {
    let mut res = vec![0; n];
    File::open(filename)?.read_exact(&mut res)?;
    Ok(res)
}
