pub mod option_impls {
    use crate::*;

    pub struct OptionKind;

    impl Generic1 for OptionKind {
        type Rep1<A> = Option<A>;
    }

    impl<A> Kinded1<A> for Option<A> {
        type Kind1 = OptionKind;
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
        fn bind<B, F: FnOnce(A) -> Apply1<Self::Kind1, B>>(self, f: F) -> Apply1<Self::Kind1, B> {
            self.and_then(f)
        }
    }
}

#[cfg(test)]
mod option_tests {
    use crate::*;
    #[cfg(feature = "no_std")]
    use crate::fixed_string::*;

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
        use crate::*;

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
