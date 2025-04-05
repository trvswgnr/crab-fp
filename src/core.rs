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
//! - `Bifunctor` - Extends `Kinded2` with the ability to map over two type
//!   parameters independently
//!
//! These traits form a hierarchy (Applicative extends Functor) and enable
//! composable, type-safe functional programming patterns in Rust.

/// A trait representing a type constructor of kind `* -> *`.
///
/// This provides a way to implement higher-kinded types in Rust's type system.
/// Type constructors take a type and produce another type, allowing for
/// generic container types like `Vec<T>` or `Option<T>`.
pub trait TypeCtor1 {
    type Type<A>;
}

/// A trait representing a type constructor of kind `* -> * -> *`.
///
/// This provides a way to implement binary higher-kinded types in Rust's type system.
/// Binary type constructors take two types and produce another type, like `Result<T, E>`
/// or `Either<L, R>`.
pub trait TypeCtor2 {
    type Type<A, B>;
}

/// Associates a concrete type with its unary type constructor.
///
/// This trait allows a type to specify its "kind" - the type constructor
/// that, when applied to type parameter `A`, produces this type.
/// It's a key component for implementing higher-kinded polymorphism.
pub trait Kinded1<A> {
    type Kind: TypeCtor1<Type<A> = Self>;
}

/// Associates a concrete type with its binary type constructor.
///
/// This trait allows a type to specify its "kind" - the type constructor
/// that, when applied to type parameters `A` and `B`, produces this type.
/// Enables higher-kinded polymorphism for types with two type parameters.
pub trait Kinded2<A, B> {
    type Kind: TypeCtor2<Type<A, B> = Self>;
}

/// Applies a unary type constructor to a type parameter.
///
/// This type alias simplifies the syntax of type application, making
/// higher-kinded type patterns more readable and concise.
pub type Apply1<F, A> = <F as TypeCtor1>::Type<A>;

/// Applies a binary type constructor to two type parameters.
///
/// This type alias simplifies the syntax of type application, making
/// higher-kinded type patterns more readable and concise when working
/// with binary type constructors.
pub type Apply2<F, A, B> = <F as TypeCtor2>::Type<A, B>;

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
pub trait Functor<A>: Kinded1<A> {
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
    fn fmap<B, M: FnMut(A) -> B>(self, f: M) -> Apply1<Self::Kind, B>;
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
    fn pure(b: A) -> Apply1<Self::Kind, A>;

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
    fn apply<B, F: FnMut(A) -> B>(self, ff: Apply1<Self::Kind, F>) -> Apply1<Self::Kind, B>;
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
    fn bind<B, F: FnMut(A) -> Apply1<Self::Kind, B>>(self, f: F) -> Apply1<Self::Kind, B>;
}

/// A trait representing types that can be mapped over in two dimensions (bifunctors).
///
/// Bifunctors are types with two type parameters, both of which can be mapped over
/// independently. Common examples include `Result<T, E>` and `Either<L, R>`.
///
/// Laws:
/// - Identity: `x.bimap(identity, identity) == x`
/// - Composition: `x.bimap(f1 . g1, f2 . g2) == x.bimap(g1, g2).bimap(f1, f2)`
///
/// # Type Parameters
/// * `A` - The type of first values contained in this bifunctor
/// * `B` - The type of second values contained in this bifunctor
pub trait Bifunctor<A, B>: Kinded2<A, B> {
    /// Maps functions over both type parameters of the bifunctor.
    ///
    /// # Parameters
    /// * `f` - A function that transforms values of type `A` into values of type `C`
    /// * `g` - A function that transforms values of type `B` into values of type `D`
    ///
    /// # Returns
    /// A new bifunctor containing the transformed values.
    fn bimap<C, D, F: FnMut(A) -> C, G: FnMut(B) -> D>(self, f: F, g: G)
    -> Apply2<Self::Kind, C, D>;

    /// Maps a function over the first type parameter of the bifunctor.
    ///
    /// # Parameters
    /// * `f` - A function that transforms values of type `A` into values of type `C`
    ///
    /// # Returns
    /// A new bifunctor with the first type parameter transformed.
    fn first<C, F: FnMut(A) -> C>(self, f: F) -> Apply2<Self::Kind, C, B>;

    /// Maps a function over the second type parameter of the bifunctor.
    ///
    /// # Parameters
    /// * `g` - A function that transforms values of type `B` into values of type `D`
    ///
    /// # Returns
    /// A new bifunctor with the second type parameter transformed.
    fn second<D, G: FnMut(B) -> D>(self, g: G) -> Apply2<Self::Kind, A, D>;
}