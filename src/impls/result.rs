pub mod result_impls {
    use crate::*;

    pub struct ResultKind<E>(std::marker::PhantomData<E>);

    impl<E> Generic1 for ResultKind<E> {
        type Rep1<A> = Result<A, E>;
    }

    impl<A, E> Kinded1<A> for Result<A, E> {
        type Kind1 = ResultKind<E>;
    }

    pub struct ResultKind2;

    impl Generic2 for ResultKind2 {
        type Rep2<A, B> = Result<A, B>;
    }

    impl<A, E> Kinded2<A, E> for Result<A, E> {
        type Kind2 = ResultKind2;
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
        fn bind<B, F: FnOnce(A) -> Result<B, E>>(self, f: F) -> Result<B, E> {
            self.and_then(f)
        }
    }

    impl<A, C> Bifunctor<A, C> for Result<A, C> {
        fn bimap<B, D, F: FnMut(A) -> B, G: FnMut(C) -> D>(
            self,
            mut f: F,
            mut g: G,
        ) -> Result<B, D> {
            match self {
                Ok(a) => Ok(f(a)),
                Err(c) => Err(g(c)),
            }
        }

        fn first<B, F: FnMut(A) -> B>(self, mut f: F) -> Result<B, C> {
            match self {
                Ok(a) => Ok(f(a)),
                Err(b) => Err(b),
            }
        }

        fn second<D, G: FnMut(C) -> D>(self, mut g: G) -> Result<A, D> {
            match self {
                Ok(a) => Ok(a),
                Err(c) => Err(g(c)),
            }
        }
    }
}

#[cfg(test)]
mod result_tests {
    use crate::*;
    #[cfg(feature = "no_std")]
    use crate::fixed_string::*;

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
        use crate::*;

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
            let result1: Result<i32, &str> = Ok(3);
            let result2 = result1.bind(|x| Ok(square(x)));
            assert_eq!(result2, Ok(9));

            let result1: Result<i32, &str> = Err("error");
            let result2 = result1.bind(|x| Ok(square(x)));
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

    mod bifunctor {
        use super::*;

        #[test]
        fn bimap() {
            // Test Ok case
            let r: Result<i32, &str> = Ok(5);
            let result = r.bimap(|x| x * 2, |s: &str| s.to_string());
            assert_eq!(result, Ok(10));

            // Test Err case
            let r: Result<i32, &str> = Err("failed");
            let result = r.bimap(|x| x * 2, |s: &str| s.to_string());
            assert_eq!(result, Err("failed".to_string()));
        }

        #[test]
        fn first() {
            // Test Ok case
            let r: Result<i32, &str> = Ok(5);
            let result = r.first(|x| x * 2);
            assert_eq!(result, Ok(10));

            // Test Err case
            let r: Result<i32, &str> = Err("failed");
            let result = r.first(|x| x * 2);
            assert_eq!(result, Err("failed"));
        }

        #[test]
        fn second() {
            // Test Ok case
            let r: Result<i32, &str> = Ok(5);
            let result = r.second(|s: &str| s.to_string());
            assert_eq!(result, Ok(5));

            // Test Err case
            let r: Result<i32, &str> = Err("failed");
            let result = r.second(|s: &str| s.to_string());
            assert_eq!(result, Err("failed".to_string()));
        }

        #[test]
        #[cfg(not(feature = "no_std"))]
        fn identity_law() {
            // Identity law: bimap id id = id
            let ok_val: Result<i32, &str> = Ok(5);
            let err_val: Result<i32, &str> = Err("error");

            assert_eq!(ok_val.bimap(identity, |s: &str| s), ok_val);
            assert_eq!(err_val.bimap(identity, |s: &str| s), err_val);
        }

        #[test]
        #[cfg(not(feature = "no_std"))]
        fn composition_law() {
            // Composition law: bimap (f . g) (h . i) = bimap f h . bimap g i
            let f = |x: i32| x.to_string();
            let g = |x: i32| x * 2;
            let h = |s: String| format!("Error: {}", s);
            let i = |s: &str| format!("{} occurred", s);

            // Test with Ok value
            let r: Result<i32, &str> = Ok(5);

            // Left side: bimap (f . g) (h . i)
            let left = r.bimap(|x| f(g(x)), |s: &str| h(i(s)));

            // Right side: bimap f h . bimap g i
            let right = r.bimap(g, i).bimap(f, h);

            assert_eq!(left, right);

            // Test with Err value
            let r: Result<i32, &str> = Err("failed");

            // Left side: bimap (f . g) (h . i)
            let left = r.bimap(|x| f(g(x)), |s: &str| h(i(s)));

            // Right side: bimap f h . bimap g i
            let right = r.bimap(g, i).bimap(f, h);

            assert_eq!(left, right);
        }
    }
}
