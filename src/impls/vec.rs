#[cfg(not(feature = "no_std"))]
pub mod vec_impls {
    use crate::*;

    pub struct VecKind;

    impl Generic1 for VecKind {
        type Rep1<A> = Vec<A>;
    }

    impl<A> Kinded1<A> for Vec<A> {
        type Kind1 = VecKind;
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
        fn bind<B, F: FnMut(A) -> Apply1<Self::Kind1, B>>(self, f: F) -> Apply1<Self::Kind1, B> {
            self.into_iter().flat_map(f).collect()
        }
    }
}

#[cfg(test)]
#[cfg(not(feature = "no_std"))]
mod vec_tests {
    mod functor {
        use crate::*;

        #[test]
        fn fmap() {
            let v = vec![1, 2, 3];
            let mapped = v.fmap(multiply_by_two);
            assert_eq!(mapped, vec![2, 4, 6]);
        }
    }

    mod applicative {
        use crate::*;

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
        use crate::*;

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
