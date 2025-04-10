//! This module provides specific implementations of functional programming
//! typeclasses for standard Rust types.
//!
//! It includes implementations of:
//! - `Functor` - for mapping over contained values
//! - `Applicative` - for applying functions wrapped in a context to values in
//!   the same context
//!
//! Implementations are provided for common types:
//! - `Option<T>`
//! - `Result<T, E>`
//! - `Vec<T>`
//!
//! Each implementation comes with test cases validating both the basic
//! functionality and the typeclass laws (identity, composition, homomorphism,
//! etc).

pub mod option;
pub mod result;
pub mod vec;
