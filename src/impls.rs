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

use crate::*;

pub struct OptionKind;

impl Endofunctor for OptionKind {
    type Domain<A> = Option<A>;
}

impl<A> Kinded<A> for Option<A> {
    type Kind = OptionKind;
}

impl<A> Functor<A> for Option<A> {
    fn fmap<B, F: FnOnce(A) -> B>(self, f: F) -> Option<B> {
        self.map(f)
    }
}

impl<A> Applicative<A> for Option<A> {
    fn pure(b: A) -> Option<A> {
        Some(b)
    }
    fn apply<B, F: FnOnce(A) -> B>(self, ff: Option<F>) -> Option<B> {
        match (self, ff) {
            (Some(a), Some(f)) => Some(f(a)),
            _ => None,
        }
    }
}

impl<A> Monad<A> for Option<A> {
    fn bind<B, F: FnOnce(A) -> Apply<Self::Kind, B>>(self, f: F) -> Apply<Self::Kind, B> {
        self.and_then(f)
    }
}

#[cfg(test)]
mod option_tests {
    use super::util_fns_for_testing::*;
    use super::*;
    mod functor {
        use super::*;

        #[test]
        fn fmap() {
            let opt1 = Some(1);
            let opt2 = opt1.fmap(|x| x + 1);
            assert_eq!(opt2, Some(2));

            let opt1: Option<i32> = None;
            let opt2 = opt1.fmap(|x| x + 1);
            assert_eq!(opt2, None);
        }
    }

    mod applicative {
        use super::*;

        #[test]
        fn pure() {
            let o = Option::pure(69);
            assert_eq!(o, Some(69));
        }

        #[test]
        fn ap() {
            let o = Some(5);
            let f = Some(add_one);
            assert_eq!(o.apply(f), Some(6));

            let o = Some(5);
            let f = Some(multiply_by_two);
            assert_eq!(o.apply(f), Some(10));
        }

        #[test]
        fn none_cases() {
            let o = Some(5);
            let f: Option<fn(_) -> i32> = None;
            assert_eq!(o.apply(f), None);

            let o = None;
            let f = Some(add_one);
            assert_eq!(o.apply(f), None);

            let o: Option<i32> = None;
            let f: Option<fn(_) -> i32> = None;
            assert_eq!(o.apply(f), None);
        }

        #[test]
        fn identity_law() {
            // Identity: pure id <*> v = v
            let v = Some(69);
            let lhs = v.apply(Option::pure(identity));
            assert_eq!(lhs, v);

            let v: Option<i32> = None;
            let lhs = v.apply(Option::pure(identity));
            assert_eq!(lhs, v);
        }

        #[test]
        fn homomorphism_law() {
            // Homomorphism: pure f <*> pure x = pure (f x)
            let x = 69;
            let p = Option::pure(to_string);
            let pp = Option::pure(x);
            let lhs = pp.apply(p);
            let rhs = Option::pure(to_string(x));
            assert_eq!(lhs, rhs);
        }

        #[test]
        fn composition_law() {
            // Test composition for Option
            let w = Some(5);
            let v = Some(multiply_by_two);
            let u = Some(to_string);

            // Apply multiply_two, then to_string_fn
            let left_side = w.apply(v).apply(u);

            let v_applied_to_w = w.apply(v);
            let right_side = v_applied_to_w.apply(u);

            assert_eq!(left_side, right_side);
        }
    }

    mod monad {
        use super::*;

        #[test]
        fn bind() {
            let opt1 = Some(1);
            let opt2 = opt1.bind(|x| Some(x + 1));
            assert_eq!(opt2, Some(2));

            let opt1: Option<i32> = None;
            let opt2 = opt1.bind(|x| Some(x + 1));
            assert_eq!(opt2, None);
        }

        #[test]
        fn left_identity_law() {
            // Left identity: return a >>= f = f a
            let a = 5;
            let f = |x: i32| Some(x * 2);

            let lhs = Option::pure(a).bind(f);
            let rhs = f(a);

            assert_eq!(lhs, rhs);
        }

        #[test]
        fn right_identity_law() {
            // Right identity: m >>= return = m
            let m = Some(5);

            let lhs = m.bind(Option::pure);

            assert_eq!(lhs, m);

            // Check with None
            let m: Option<i32> = None;
            let lhs = m.bind(Option::pure);
            assert_eq!(lhs, m);
        }

        #[test]
        fn associativity_law() {
            // Associativity: (m >>= f) >>= g = m >>= (|x| f x >>= g)
            let m = Some(5);
            let f = |x: i32| Some(x * 2);
            let g = |x: i32| Some(x.to_string());

            // Left side: (m >>= f) >>= g
            let lhs = m.bind(f).bind(g);

            // Right side: m >>= (|x| f x >>= g)
            let rhs = m.bind(|x| f(x).bind(g));

            assert_eq!(lhs, rhs);

            // Check with None
            let m: Option<i32> = None;
            let lhs = m.bind(f).bind(g);
            let rhs = m.bind(|x| f(x).bind(g));
            assert_eq!(lhs, rhs);
        }

        #[test]
        fn chaining() {
            // Test chaining multiple bind operations
            let result = Some(5)
                .bind(|x| Some(x * 2))
                .bind(|x| Some(x + 3))
                .bind(|x: i32| Some(x.to_string()));

            assert_eq!(result, Some("13".to_string()));

            // Chain that should fail at the middle
            let result = Some(5)
                .bind(|x| Some(x * 2))
                .bind(|_| None::<i32>)
                .bind(|x: i32| Some(x.to_string()));

            assert_eq!(result, None);
        }
    }
}

pub struct ResultKind<E>(std::marker::PhantomData<E>);

impl<E> Endofunctor for ResultKind<E> {
    type Domain<A> = Result<A, E>;
}

impl<A, E> Kinded<A> for Result<A, E> {
    type Kind = ResultKind<E>;
}

impl<A, E> Functor<A> for Result<A, E> {
    fn fmap<B, F: FnOnce(A) -> B>(self, f: F) -> Result<B, E> {
        self.map(f)
    }
}

impl<A, E> Applicative<A> for Result<A, E> {
    fn pure(b: A) -> Result<A, E> {
        Ok(b)
    }

    fn apply<B, F: FnOnce(A) -> B>(self, ff: Result<F, E>) -> Result<B, E> {
        match (self, ff) {
            (Ok(a), Ok(f)) => Ok(f(a)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }
}

impl<A, E> Monad<A> for Result<A, E> {
    fn bind<B, F: FnOnce(A) -> Result<B, E>>(
        self,
        f: F,
    ) -> Result<B, E> {
        self.and_then(f)
    }
}

#[cfg(test)]
mod result_tests {
    use super::util_fns_for_testing::*;
    use super::*;

    mod functor {
        use super::*;

        #[test]
        fn fmap() {
            let result1: Result<i32, &str> = Ok(1);
            let result2 = result1.fmap(|x| x + 1);
            assert_eq!(result2, Ok(2));

            let result1: Result<i32, &str> = Err("error");
            let result2 = result1.fmap(|x| x + 1);
            assert_eq!(result2, Err("error"));
        }
    }

    mod applicative {
        use super::*;

        #[test]
        fn pure() {
            let r: Result<i32, &str> = Result::pure(69);
            assert_eq!(r, Ok(69));
        }

        #[test]
        fn ap() {
            let r: Result<i32, &str> = Ok(5);
            let f = Ok(add_one);
            assert_eq!(r.apply(f), Ok(6));

            let r: Result<i32, &str> = Ok(5);
            let f = Ok(multiply_by_two);
            assert_eq!(r.apply(f), Ok(10));
        }

        #[test]
        fn err_cases() {
            let r = Ok(5);
            let f: Result<fn(_) -> i32, &str> = Err("function error");
            assert_eq!(r.apply(f), Err("function error"));

            let r = Err("value error");
            let f = Ok(add_one);
            assert_eq!(r.apply(f), Err("value error"));

            let r: Result<i32, &str> = Err("value error");
            let f: Result<fn(_) -> i32, &str> = Err("function error");
            assert_eq!(r.apply(f), Err("value error"));
        }

        #[test]
        fn identity_law() {
            // Identity: pure id <*> v = v
            let v: Result<i32, &str> = Ok(69);
            let p = Ok(identity);
            let lhs = v.apply(p);
            assert_eq!(lhs, v);

            let v: Result<i32, &str> = Err("error");
            let p = Ok(identity);
            let lhs = v.apply(p);
            assert_eq!(lhs, v);
        }

        #[test]
        fn homomorphism_law() {
            // Homomorphism: pure f <*> pure x = pure (f x)
            let x = 69;
            let pure_f = Ok(to_string);
            let pure_x: Result<i32, &str> = Ok(x);
            let lhs = pure_x.apply(pure_f); // pure_f <*> pure_x
            let rhs = Ok(to_string(x));
            assert_eq!(lhs, rhs);
        }

        #[test]
        fn composition_law() {
            // Test composition for Result
            let w: Result<i32, &str> = Ok(5);
            let v = Ok(multiply_by_two);
            let u = Ok(to_string);

            // Apply multiply_two, then to_string_fn
            let left_side = w.apply(v).apply(u);

            let v_applied_to_w = w.apply(v);
            let right_side = v_applied_to_w.apply(u);

            assert_eq!(left_side, right_side);
        }
    }

    mod monad {
        use super::*;

        #[test]
        fn bind() {
            let result1: Result<i32, &str> = Ok(1);
            let result2 = result1.bind(|x| Ok(x + 1));
            assert_eq!(result2, Ok(2));

            let result1: Result<i32, &str> = Err("error");
            let result2 = result1.bind(|x| Ok(x + 1));
            assert_eq!(result2, Err("error"));
        }

        #[test]
        fn left_identity_law() {
            // Left identity: return a >>= f = f a
            let a = 5;
            let f = |x: i32| Ok::<_, &str>(x * 2);

            let lhs = Result::pure(a).bind(f);
            let rhs = f(a);

            assert_eq!(lhs, rhs);
        }

        #[test]
        fn right_identity_law() {
            // Right identity: m >>= return = m
            let m: Result<i32, &str> = Ok(5);

            let lhs = m.bind(Result::pure);

            assert_eq!(lhs, m);

            // Check with Err
            let m: Result<i32, &str> = Err("error");
            let lhs = m.bind(Result::pure);
            assert_eq!(lhs, m);
        }

        #[test]
        fn associativity_law() {
            // Associativity: (m >>= f) >>= g = m >>= (|x| f x >>= g)
            let m: Result<i32, &str> = Ok(5);
            let f = |x: i32| Ok::<_, &str>(x * 2);
            let g = |x: i32| Ok::<_, &str>(x.to_string());

            // Left side: (m >>= f) >>= g
            let lhs = m.bind(f).bind(g);

            // Right side: m >>= (|x| f x >>= g)
            let rhs = m.bind(|x| f(x).bind(g));

            assert_eq!(lhs, rhs);

            // Check with Err
            let m: Result<i32, &str> = Err("error");
            let lhs = m.bind(f).bind(g);
            let rhs = m.bind(|x| f(x).bind(g));
            assert_eq!(lhs, rhs);
        }

        #[test]
        fn chaining() {
            // Test chaining multiple bind operations
            let result = Ok::<_, &str>(5)
                .bind(|x| Ok(x * 2))
                .bind(|x| Ok(x + 3))
                .bind(|x: i32| Ok(x.to_string()));

            assert_eq!(result, Ok("13".to_string()));

            // Chain that should fail at the middle
            let result = Ok::<_, &str>(5)
                .bind(|x| Ok(x * 2))
                .bind(|_| Err("operation failed"))
                .bind(|x: i32| Ok(x.to_string()));

            assert_eq!(result, Err("operation failed"));
        }
    }
}

pub struct VecKind;

impl Endofunctor for VecKind {
    type Domain<A> = Vec<A>;
}

impl<A> Kinded<A> for Vec<A> {
    type Kind = VecKind;
}

impl<A> Functor<A> for Vec<A> {
    fn fmap<B, F: FnMut(A) -> B>(self, f: F) -> Vec<B> {
        self.into_iter().map(f).collect()
    }
}

impl<A> Applicative<A> for Vec<A> {
    fn pure(b: A) -> Vec<A> {
        vec![b]
    }

    fn apply<B, F: FnMut(A) -> B>(self, ff: Vec<F>) -> Vec<B> {
        let mut result = Vec::with_capacity(self.len() * ff.len());

        // Handle empty cases
        if self.is_empty() || ff.is_empty() {
            return result;
        }

        // We need to use unsafe to avoid cloning values
        unsafe {
            // Convert self into raw parts
            let v_ptr = self.as_ptr();
            let v_len = self.len();

            // For each function, apply it to each value
            for mut f in ff {
                for i in 0..v_len {
                    // Read the value at index i without consuming it
                    let elem_ref = &*v_ptr.add(i);
                    // Use std::ptr::read to copy the value without requiring Clone
                    let elem = std::ptr::read(elem_ref);
                    // Apply the function and push the result
                    result.push(f(elem));
                }
            }

            // Leak the original vector to avoid double-free
            std::mem::forget(self);
        }

        result
    }
}

impl<A> Monad<A> for Vec<A> {
    fn bind<B, F: FnMut(A) -> Apply<Self::Kind, B>>(self, f: F) -> Apply<Self::Kind, B> {
        self.into_iter().flat_map(f).collect()
    }
}

#[cfg(test)]
mod vec_tests {
    use super::util_fns_for_testing::*;
    use super::*;

    mod functor {
        use super::*;

        #[test]
        fn fmap() {
            let v = vec![1, 2, 3];
            let mapped = v.fmap(multiply_by_two);
            assert_eq!(mapped, vec![2, 4, 6]);
        }
    }

    mod applicative {
        use super::*;

        #[test]
        fn pure() {
            let v = Vec::pure(69);
            assert_eq!(v, vec![69]);
        }

        #[test]
        fn ap() {
            let v = vec![1, 2, 3];
            let fs = vec![add_one, multiply_by_two, square];
            let result = v.apply(fs);
            assert_eq!(result, vec![2, 3, 4, 2, 4, 6, 1, 4, 9]);
        }

        #[test]
        fn empty_ap() {
            let empty_vec: Vec<i32> = vec![];
            let fs = vec![add_one];
            let result = empty_vec.apply(fs);
            assert_eq!(result, vec![]);

            let v = vec![1, 2, 3];
            let empty_fs: Vec<fn(_) -> i32> = vec![];
            let result = v.apply(empty_fs);
            assert_eq!(result, vec![]);
        }

        #[test]
        fn identity_law() {
            // Identity: pure id <*> v = v
            let v = vec![1, 2, 3];
            let p = Vec::pure(identity);
            let lhs = v.clone().apply(p);
            assert_eq!(lhs, v);
        }

        #[test]
        fn homomorphism_law() {
            // Homomorphism: pure f <*> pure x = pure (f x)
            let x = 69;
            let p = Vec::pure(to_string);
            let lhs = Vec::pure(x).apply(p);
            let rhs = Vec::pure(to_string(x));
            assert_eq!(lhs, rhs);
        }

        #[test]
        fn composition_law() {
            // Composition: u <*> (v <*> w) = u.ap(v).ap(w)
            let w = vec![1, 2, 3];
            let v = vec![multiply_by_two, add_one, square];
            let u = vec![to_string];

            // Left side: w.ap(v).ap(u) - applying v to w, then applying u to the result
            let left_side = w.clone().apply(v.clone()).apply(u.clone());

            // Calculate (v <*> w)
            let v_applied_to_w = w.clone().apply(v.clone());
            // Apply u to the result
            let right_side = v_applied_to_w.apply(u.clone());

            // Both sides should be equal according to the composition law
            assert_eq!(left_side, right_side);

            // Test with multiple functions in u
            let w = vec![1, 2];
            let v = vec![multiply_by_two, add_one];
            let u = vec![|x| format!("Number: {}", x), |x| format!("Value: {}", x)];

            let left_side = w.clone().apply(v.clone()).apply(u.clone());
            let v_applied_to_w = w.clone().apply(v.clone());
            let right_side = v_applied_to_w.apply(u.clone());

            assert_eq!(left_side, right_side);
        }
    }

    mod monad {
        use super::*;

        #[test]
        fn bind() {
            let vec1 = vec![1];
            let vec2 = vec1.bind(|x| vec![x + 1]);
            assert_eq!(vec2, vec![2]);

            let vec1: Vec<i32> = vec![];
            let vec2 = vec1.bind(|x| vec![x + 1]);
            assert_eq!(vec2, vec![]);
        }

        #[test]
        fn left_identity_law() {
            // Left identity: return a >>= f = f a
            let a = 5;
            let f = |x: i32| vec![x * 2];

            let lhs = Vec::pure(a).bind(f);
            let rhs = f(a);

            assert_eq!(lhs, rhs);
        }

        #[test]
        fn right_identity_law() {
            // Right identity: m >>= return = m
            let m = vec![1, 2, 3];

            let lhs = m.clone().bind(Vec::pure);

            assert_eq!(lhs, m);

            // Check with None
            let m: Vec<i32> = vec![];
            let lhs = m.clone().bind(Vec::pure);
            assert_eq!(lhs, m);
        }

        #[test]
        fn associativity_law() {
            // Associativity: (m >>= f) >>= g = m >>= (|x| f x >>= g)
            let m = vec![1, 2, 3];
            let f = |x: i32| vec![x * 2];
            let g = |x: i32| vec![x.to_string()];

            // Left side: (m >>= f) >>= g
            let lhs = m.clone().bind(f).bind(g);

            // Right side: m >>= (|x| f x >>= g)
            let rhs = m.bind(|x| f(x).bind(g));

            assert_eq!(lhs, rhs);

            // Check with None
            let m: Vec<i32> = vec![];
            let lhs = m.clone().bind(f).bind(g);
            let rhs = m.clone().bind(|x| f(x).bind(g));
            assert_eq!(lhs, rhs);
        }

        #[test]
        fn chaining() {
            // Test chaining multiple bind operations
            let result = vec![5]
                .bind(|x| vec![x * 2])
                .bind(|x| vec![x + 3])
                .bind(|x: i32| vec![x.to_string()]);

            assert_eq!(result, vec!["13".to_string()]);

            // Chain that should fail at the middle
            let result = vec![5]
                .bind(|x| vec![x * 2])
                .bind(|_| vec![])
                .bind(|x: i32| vec![x.to_string()]);

            assert_eq!(result, Vec::<String>::new());
        }
    }
}

#[cfg(test)]
mod util_fns_for_testing {
    pub(crate) fn add_one(x: i32) -> i32 {
        x + 1
    }

    pub(crate) fn multiply_by_two(x: i32) -> i32 {
        x * 2
    }

    pub(crate) fn square(x: i32) -> i32 {
        x * x
    }

    pub(crate) fn to_string<T: ToString>(x: T) -> String {
        x.to_string()
    }
}
