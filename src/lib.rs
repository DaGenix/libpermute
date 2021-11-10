//! # libpermute
//!
//! libpermute is a `no_std` compatible library that provides a function
//! to permute the items of a slice.
//!
//! libpermute offers a few main features:
//!
//! 1. Stability. The permutation algorithm won't be changed without a
//! major version bump.
//!
//! 2. Re-implementability. It should be relatively straightforward to re-implement the algorithm
//! this library uses in another library or in another language.
//!
//! 3. Strength. Given an unpredictable permute_key, the permutation should also
//! be unpredictable - assuming that no one breaks the [Sha-256](https://en.wikipedia.org/wiki/SHA-2)
//! or [ChaChar20](https://en.wikipedia.org/wiki/Salsa20) algorithms.
//!
//! # Example
//!
//! ```
//! use libpermute::permute;
//!
//! fn main() {
//!     const KEY: &'static [u8] = &[0, 1, 2, 3, 4, 5, 6, 7];
//!     const INPUT: &str = "Hello World!";
//!
//!     let mut buff = [0u8; INPUT.len()];
//!     buff.copy_from_slice(INPUT.as_bytes());
//!
//!     permute(KEY, &mut buff);
//!
//!     println!("RESULT: '{}'", std::str::from_utf8(&buff).unwrap());
//! }
//! ```
//!
//! ## No_std
//!
//! No_std mode may be activated by disabling the "std" feature.
//!
//! ## License
//!
//! This project is licensed under either of
//!
//! * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
//!   <https://www.apache.org/licenses/LICENSE-2.0>)
//! * MIT license ([LICENSE-MIT](LICENSE-MIT) or
//!   <https://opensource.org/licenses/MIT>)
//!
//! at your option.

#![cfg_attr(not(feature = "std"), no_std)]

mod permute;
mod usize_generator;

#[cfg(test)]
mod tests;

use static_assertions as sa;
sa::const_assert!(u64::BITS >= usize::BITS);

pub use permute::{permute, PermuteKey};
pub use usize_generator::PermuteKeyData;
