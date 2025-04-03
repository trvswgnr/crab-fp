//! This module defines the core traits for functional programming typeclasses.
//!
//! It provides the foundation for implementing functional programming\
//! abstractions in Rust:
//! - `Kinded` - Associates container types with their "kind" for proper type
//!   resolution
//! - `Functor` - Represents types that can be mapped over (supporting `fmap`
//!   operation)
//! - `Applicative` - Extends `Functor` with the ability to apply functions
//!   contained within a context to values in the same context
//! - `Monad` - Extends `Applicative` with the ability to bind functions to
//!   values in a context
//!
//! These traits form a hierarchy (Applicative extends Functor) and enable
//! composable, type-safe functional programming patterns in Rust.

/// A trait that allows associating a container type with its "kind".
///
/// This trait solves the higher-kinded types problem in Rust by providing
/// a way to associate a generic container type with a concrete type that
/// represents its "kind". This enables proper type resolution when working
/// with higher-order abstractions like Functors and Applicatives.
pub trait Kinded {
    /// The container type parametrized by type `A`.
    ///
    /// This associated type represents the actual container that holds values
    /// of type `A`. For example, `Option<A>`, `Result<A, E>`, or `Vec<A>`.
    type Container<A>;
}

/// A trait representing types that can be mapped over (functors).
///
/// Functors are containers that allow applying a function to their contained
/// values while preserving the structure of the container. This trait enables
/// functional-style transformations on contained values without having to
/// manually handle the container's structure.
///
/// Laws:
/// - Identity: `x.fmap(identity) == x`
/// - Composition: `x.fmap(f).fmap(g) == x.fmap(|a| g(f(a)))`
///
/// # Type Parameters
/// * `A` - The type of values contained in this functor
pub trait Functor<A> {
    /// The concrete type representing the "kind" of this functor.
    ///
    /// This associated type must implement `Kinded` with its `Container<A>`
    /// associated type equal to `Self`.
    type Kind: Kinded<Container<A> = Self>;

    /// Maps a function over the contained value(s).
    ///
    /// Applies the function `f` to each value contained in this functor,
    /// preserving the structure of the container.
    ///
    /// # Parameters
    /// * `f` - A function that transforms values of type `A` into values of type `B`
    ///
    /// # Returns
    /// A new container of the same kind containing the transformed values.
    fn fmap<B, F: FnMut(A) -> B>(self, f: F) -> <Self::Kind as Kinded>::Container<B>;
}

/// A trait representing applicative functors.
///
/// Applicative functors extend the capabilities of functors by allowing:
/// 1. Lifting values into the applicative context (`pure`)
/// 2. Applying functions wrapped in an applicative context to values in the same context (`ap`)
///
/// Laws:
/// - Identity: `pure(id) <*> v = v`
/// - Homomorphism: `pure(f) <*> pure(x) = pure(f(x))`
/// - Interchange: `u <*> pure(y) = pure(|f| f(y)) <*> u`
/// - Composition: `pure(compose) <*> u <*> v <*> w = u <*> (v <*> w)`
///
/// # Type Parameters
/// * `A` - The type of values contained in this applicative functor
pub trait Applicative<A>: Functor<A> {
    /// Lifts a value into the applicative context.
    ///
    /// Creates a new container of the same kind holding the provided value.
    ///
    /// # Parameters
    /// * `b` - The value to lift into the applicative context
    ///
    /// # Returns
    /// A new container of the same kind containing the provided value.
    fn pure(b: A) -> <Self::Kind as Kinded>::Container<A>;

    /// Applies functions contained in an applicative context to values in this applicative context.
    ///
    /// This operation allows sequential application of functions to values, where both
    /// the functions and values are wrapped in the same type of container.
    ///
    /// # Parameters
    /// * `ff` - A container of functions that transform values of type `A` into values of type `B`
    ///
    /// # Returns
    /// A new container of the same kind containing the results of applying the functions to the values.
    fn apply<B, F: FnMut(A) -> B>(
        self,
        ff: <Self::Kind as Kinded>::Container<F>,
    ) -> <Self::Kind as Kinded>::Container<B>;
}

pub trait Monad<A>: Applicative<A> {
    fn bind<B, F: FnMut(A) -> <Self::Kind as Kinded>::Container<B>>(
        self,
        f: F,
    ) -> <Self::Kind as Kinded>::Container<B>;
}
