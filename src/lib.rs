//! # Functional Programming for Rust
//!
//! This crate provides functional programming abstractions and utilities for Rust.
//! It implements common functional programming typeclasses and utilities that enable
//! functional programming patterns while maintaining Rust's strong type safety.
//!
//! ## Main Components
//!
//! * **Core Typeclasses**: Functor, Applicative and related traits that form the
//!   foundation of the functional programming approach.
//!
//! * **Standard Type Implementations**: Ready-to-use implementations for common Rust
//!   types like Option, Result, and Vec.
//!
//! * **Utility Functions**: Helpers for function composition, currying, and other
//!   functional programming techniques.
//!
//! ## Example
//!
//! ```
//! use crab_fp::*;
//!
//! // Map over different container types using the same Functor interface
//! let opt = Some(5).fmap(|x| x * 2);  // Some(10)
//! let result: Result<i32, &str> = Ok(5).fmap(|x| x * 2);  // Ok(10)
//! ```

#![cfg_attr(feature = "no_std", no_std, feature(inherent_str_constructors))]

#[cfg(feature = "no_std")]
extern crate core as std;

#[cfg(feature = "no_std")]
pub(crate) mod fixed_string;

mod core;
pub use core::*;

mod impls;
pub use impls::*;

mod util;
pub use util::utilities::*;

#[cfg(test)]
pub(crate) use util::for_testing::*;

#[cfg(test)]
#[cfg(feature = "no_std")]
mod _no_std_test {
    #[test]
    fn test_is_no_std() {
        assert!(cfg!(feature = "no_std"));
    }
}
