use crate::{Applicative, Apply, TypeConstructor, Functor};

/// Identity trait
///
/// # Example
/// ```rust
/// use crab_fp::Identity;
///
/// let id = 1.identity();
/// assert_eq!(id, 1);
/// ```
pub trait Identity {
    fn identity(self) -> Self;
}

impl<A> Identity for A {
    fn identity(self) -> Self {
        self
    }
}

/// Identity function
///
/// # Example
/// ```rust
/// use crab_fp::identity;
///
/// let id = identity(1);
/// assert_eq!(id, 1);
/// ```
pub fn identity<A>(a: A) -> A {
    a
}

/// Composable trait
///
/// # Example
/// ```rust
/// use crab_fp::Composable;
///
/// fn add_one(x: i32) -> i32 {
///     x + 1
/// }
///
/// fn multiply_by_two(x: i32) -> i32 {
///     x * 2
/// }
///
/// let add_one_then_multiply_by_two = multiply_by_two.compose(add_one);
/// assert_eq!(add_one_then_multiply_by_two(5), 12);
/// ```
pub trait Composable<A, B> {
    fn compose<C>(self, f: fn(C) -> A) -> impl Fn(C) -> B;
}

impl<A, B, F: Fn(A) -> B> Composable<A, B> for F {
    fn compose<C>(self, f: fn(C) -> A) -> impl Fn(C) -> B {
        move |c| self(f(c))
    }
}

/// Compose two functions, right to left
///
/// # Example
/// ```rust
/// use crab_fp::compose;
///
/// fn add_one(x: i32) -> i32 {
///     x + 1
/// }
///
/// fn multiply_by_two(x: i32) -> i32 {
///     x * 2
/// }
///
/// let add_one_then_multiply_by_two = compose(multiply_by_two, add_one);
/// assert_eq!(add_one_then_multiply_by_two(5), 12);
/// ```
///
/// In a language with pipes, this would look like:
/// ```ignore
/// let add_one_and_multiply_by_two = multiply_by_two . add_one;
/// ```
pub fn compose<A, B, C>(f: fn(B) -> C, g: fn(A) -> B) -> impl Fn(A) -> C {
    move |a| f(g(a))
}

/// Pipeable trait
///
/// # Example
/// ```rust
/// use crab_fp::Pipeable;
///
/// fn add_one(x: i32) -> i32 {
///     x + 1
/// }
///
/// fn multiply_by_two(x: i32) -> i32 {
///     x * 2
/// }
///
/// let add_one_then_multiply_by_two = add_one.pipe(multiply_by_two);
/// assert_eq!(add_one_then_multiply_by_two(5), 12);
/// ```
///
/// In a language with pipes, this would look like:
/// ```ignore
/// let add_one_then_multiply_by_two = add_one |> multiply_by_two;
/// ```
pub trait Pipeable<A, B> {
    fn pipe<C>(self, f: fn(B) -> C) -> impl Fn(A) -> C;
}

impl<A, B, F: Fn(A) -> B> Pipeable<A, B> for F {
    fn pipe<C>(self, f: fn(B) -> C) -> impl Fn(A) -> C {
        move |a| f(self(a))
    }
}

/// Compose two functions, left to right
///
/// # Example
/// ```rust
/// use crab_fp::pipe;
///
/// fn add_one(x: i32) -> i32 {
///     x + 1
/// }
///
/// fn multiply_by_two(x: i32) -> i32 {
///     x * 2
/// }
///
/// let add_one_and_multiply_by_two = pipe(add_one, multiply_by_two);
/// assert_eq!(add_one_and_multiply_by_two(5), 12);
/// ```
///
/// In a language with pipes, this would look like:
/// ```ignore
/// let add_one_and_multiply_by_two = add_one |> multiply_by_two;
/// ```
pub fn pipe<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |a| g(f(a))
}

/// Flip the arguments of a function
///
/// # Example
/// ```rust
/// use crab_fp::flip;
///
/// let divide = |a: i32, b: i32| a / b;
/// let divide_flipped = flip(divide);
/// assert_eq!(divide_flipped(2, 6), 3);
/// ```
pub fn flip<A, B, C, F: Fn(A, B) -> C>(f: F) -> impl Fn(B, A) -> C {
    move |b, a| f(a, b)
}

/// Curry a function of two arguments, returning a function of one argument that returns a function of the other argument
///
/// # Example
/// ```rust
/// use crab_fp::curry;
///
/// fn add(a: i32, b: i32) -> i32 {
///     a + b
/// }
///
/// let add_curried = curry(add);
/// let add_one = add_curried(1);
/// assert_eq!(add_one(2), 3);
/// ```
pub fn curry<A, B, C>(f: fn(A, B) -> C) -> impl Fn(A) -> Box<dyn Fn(B) -> C>
where
    A: Clone + 'static, // We need to clone regardless
    B: 'static,
    C: 'static,
{
    move |a: A| {
        let a = a.clone(); // Clone here for multi-use
        Box::new(move |b: B| f(a.clone(), b))
    }
}

#[cfg(test)]
mod curry_tests {
    use super::*;

    #[test]
    fn basic() {
        let f = curry(|a, b| a + b);
        let g = f(1);
        let h = g(2);
        assert_eq!(h, 3);
    }

    #[test]
    fn multiple_calls() {
        let f = curry(|a, b| a + b);
        let g = f(1);
        let h1 = g(2);
        let h2 = g(3);
        assert_eq!(h1, 3);
        assert_eq!(h2, 4);
    }

    #[test]
    fn non_copy() {
        let f = curry(|a: String, b: &str| a + b);
        let g = f(String::from("hello"));
        let h1 = g(", world");
        let h2 = g("!");
        assert_eq!(h1, "hello, world");
        assert_eq!(h2, "hello!");
    }

    #[test]
    fn large_struct() {
        #[derive(Debug, PartialEq, Clone)]
        struct LargeStruct {
            data: [u8; 1024],
            value: i32,
        }

        let f = curry(|a: LargeStruct, b: i32| a.value + b);
        let large = LargeStruct {
            data: [0; 1024],
            value: 42,
        };
        let g = f(large);
        let h1 = g(8);
        let h2 = g(58);
        assert_eq!(h1, 50);
        assert_eq!(h2, 100);
    }
}

/// Uncurry a function of one argument that returns a function of another argument
///
/// # Example
/// ```rust
/// use crab_fp::{curry, uncurry};
///
/// let add = curry(|a, b| a + b);
/// let add_uncurried = uncurry(add);
/// assert_eq!(add_uncurried(1, 2), 3);
/// ```
pub fn uncurry<A: 'static, B: 'static, C: 'static>(
    f: impl Fn(A) -> Box<dyn Fn(B) -> C> + 'static,
) -> impl Fn(A, B) -> C {
    move |a, b| (f(a))(b)
}

/// Convert a value of type Option<T> to Result<T, E> with a default error
pub fn option_to_result<T, E>(opt: Option<T>, err: E) -> Result<T, E> {
    match opt {
        Some(v) => Ok(v),
        None => Err(err),
    }
}

/// A function that applies a function to a functor.
///
/// This function takes a functor and a function, and applies the function to
/// each element of the functor.
///
/// # Type Parameters
/// * `A` - The type of values contained in the functor
/// * `B` - The type of values returned by the function
/// * `FA` - The type of the functor
/// * `F` - The type of the function
///
/// # Returns
/// A new functor of the same kind containing the transformed values.
///
/// # Example
/// ```
/// use crab_fp::fmap;
///
/// let x = Some(5);
/// let f = |x| x * 2;
/// let y = fmap(x, f);
/// assert_eq!(y, Some(10));
///
/// let x = vec![1, 2, 3];
/// let f = |x| x * 2;
/// let y = fmap(x, f);
/// assert_eq!(y, vec![2, 4, 6]);
///
/// let x = Result::<i32, &str>::Ok(5);
/// let f = |x| x * 2;
/// let y = fmap(x, f);
/// assert_eq!(y, Ok(10));
/// ```
pub fn fmap<A, B, FA: Functor<A>, F: FnMut(A) -> B>(f: FA, g: F) -> Apply<FA::Kind, B> {
    f.fmap(g)
}

/// A function that lifts a value into an applicative context.
///
/// This function takes a value and an applicative functor, and lifts the value into the
/// applicative context of the functor.
///
/// # Type Parameters
/// * `A` - The type of the value to lift
/// * `FA` - The type of the applicative functor
///
/// # Returns
/// A new applicative functor containing the lifted value.
///
/// # Example
/// ```
/// use crab_fp::pure;
///
/// let y = pure::<i32, Option<_>>(5);
/// assert_eq!(y, Some(5));
/// ```
pub fn pure<A, FA: Applicative<A>>(a: A) -> Apply<FA::Kind, A> {
    FA::pure(a)
}

/// Applies functions contained in an applicative context to values in another applicative context.
///
/// This function takes an applicative functor containing values and another applicative functor
/// containing functions, and applies each function to each value, collecting the results in a
/// new applicative functor.
///
/// # Type Parameters
/// * `A` - The type of values contained in the first applicative
/// * `B` - The type of values in the resulting applicative
/// * `FA` - The type of the applicative containing values
/// * `F` - The type of function that transforms A into B
///
/// # Returns
/// A new applicative functor containing the results of applying the functions to the values.
///
/// # Example
/// ```
/// use crab_fp::ap;
///
/// fn add_one(x: i32) -> i32 { x + 1 }
///
/// let x = Some(5);
/// let f = Some(add_one);
/// let y = ap(x, f);
/// assert_eq!(y, Some(6));
/// ```
pub fn ap<A, B, F, FA>(x: FA, fs: Apply<FA::Kind, F>) -> Apply<FA::Kind, B>
where
    F: FnMut(A) -> B,
    FA: Applicative<A>,
{
    x.apply::<B, F>(fs)
}

#[cfg(test)]
mod standalone_ap_tests {
    use super::*;

    #[test]
    fn test_ap() {
        fn add_one(x: i32) -> i32 {
            x + 1
        }

        let x = Some(5);
        let f = Some(add_one);
        let y = ap(x, f);
        assert_eq!(y, Some(6));

        let x = vec![1, 2, 3];
        let f = vec![add_one];
        let y = ap(x, f);
        assert_eq!(y, vec![2, 3, 4]);

        let x = vec![1, 2, 3];
        let f = vec![add_one, add_one];
        let y = ap(x, f);
        assert_eq!(y, vec![2, 3, 4, 2, 3, 4]);

        let x: Result<i32, &str> = Ok(5);
        let f = Ok(add_one);
        let y = ap(x, f);
        assert_eq!(y, Ok(6));
    }
}

#[cfg(test)]
mod standalone_pure_tests {
    use super::*;

    #[test]
    fn test_pure() {
        let y = pure::<i32, Option<_>>(5);
        assert_eq!(y, Some(5));

        let y = pure::<i32, Vec<_>>(5);
        assert_eq!(y, vec![5]);

        let y = pure::<i32, Result<_, &str>>(5);
        assert_eq!(y, Ok(5));
    }
}

/// Converts a function expression to a function pointer.
///
/// This macro helps with type inference when you need to pass a function
/// as a function pointer rather than a closure. It's particularly useful
/// when working with higher-order functions in this crate.
///
/// # Example
///
/// ```
/// use crab_fp::*;
///
/// fn add_one(x: i32) -> i32 { x + 1 }
///
/// let f = fn_ptr!(add_one);
/// let option = Some(5);
/// let result = option.apply(Some(f));  // Some(6)
/// ```
#[macro_export]
macro_rules! fn_ptr {
    ($fn:expr) => {
        $fn as fn(_) -> _
    };
}
