use crab_fp::*;

/// This example demonstrates basic usage of the functional programming primitives.
/// It shows how to use Functor and Applicative with different container types.
fn main() {
    println!("=== Basic Functor Operations ===");

    // Option functor example
    let opt = Some(5);
    let mapped_opt = opt.fmap(|x| x * 2);
    println!("Option mapped: {:?}", mapped_opt); // Some(10)

    #[cfg(not(feature = "no_std"))]
    {
        // Vec functor example
        let v = vec![1, 2, 3];
        let mapped_v = v.fmap(|x| x * 2);
        println!("Vec mapped: {:?}", mapped_v); // [2, 4, 6]
    }

    // Result functor example
    let result: Result<i32, &str> = Ok(5);
    let mapped_result = result.fmap(|x| x * 2);
    println!("Result mapped: {:?}", mapped_result); // Ok(10)

    println!("\n=== Basic Applicative Operations ===");

    // Option applicative example
    let value = Some(5);
    let function = Some(|x: i32| x + 3);
    let applied = value.apply(function);
    println!("Option applied: {:?}", applied); // Some(8)

    #[cfg(not(feature = "no_std"))]
    {
        // Vec applicative example
        let numbers = vec![1, 2, 3];
        let functions: Vec<fn(i32) -> i32> = vec![|x| x + 1, |x| x * 2];
        let applied = numbers.apply(functions);
        println!("Vec applied: {:?}", applied); // [2, 3, 4, 2, 4, 6]
    }

    // Creating values with pure
    let pure_option = Option::<i32>::pure(42);
    println!("Pure option: {:?}", pure_option); // Some(42)
}
