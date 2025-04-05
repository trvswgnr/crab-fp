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

pub struct Never;

pub trait TypeConstructor {
    type Type<A, B, C, D>;
}

pub trait Kinded<A, B = Never, C = Never, D = Never> {
    type Kind: TypeConstructor<Type<A, B, C, D> = Self>;
}

pub type Apply<F, A, B = Never, C = Never, D = Never> = <F as TypeConstructor>::Type<A, B, C, D>;

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
pub trait Functor<A>: Kinded<A> {
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
    fn fmap<B, M: FnMut(A) -> B>(self, f: M) -> Apply<Self::Kind, B>;
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
    fn pure(b: A) -> Apply<Self::Kind, A>;

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
    fn apply<B, F: FnMut(A) -> B>(self, ff: Apply<Self::Kind, F>) -> Apply<Self::Kind, B>;
}

/// A trait representing monads.
///
/// Monads extend the capabilities of applicative functors by providing a way to
/// sequence computations that may have effects. The `bind` operation (also known
/// as `flatMap` or `>>=`) allows chaining operations that return values wrapped
/// in the same context.
///
/// Laws:
/// - Left identity: `pure(a).bind(f) == f(a)`
/// - Right identity: `m.bind(pure) == m`
/// - Associativity: `m.bind(f).bind(g) == m.bind(|x| f(x).bind(g))`
///
/// # Type Parameters
/// * `A` - The type of values contained in this monad
pub trait Monad<A>: Applicative<A> {
    /// Binds a function to the value in this monad.
    ///
    /// This operation allows chaining computations that return values wrapped in
    /// the same context, enabling sequential processing with potential effects.
    ///
    /// # Parameters
    /// * `f` - A function that transforms values of type `A` into a new monad
    ///   containing values of type `B`
    ///
    /// # Returns
    /// A new monad of the same kind containing the results of applying the function
    /// and flattening the resulting structure.
    fn bind<B, F: FnMut(A) -> Apply<Self::Kind, B>>(self, f: F) -> Apply<Self::Kind, B>;
}
