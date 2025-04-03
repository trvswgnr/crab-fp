use crab_fp::*;

/// This example demonstrates function utilities
/// such as pipe, compose, curry, and related functionality.
fn main() {
    println!("=== Utility Examples ===\n");

    // Basic function definitions
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    fn multiply_by_two(x: i32) -> i32 {
        x * 2
    }
    fn divide_by_two(x: i32) -> i32 {
        x / 2
    }
    fn to_string(x: i32) -> String {
        format!("{}", x)
    }

    // Using pipe (left-to-right composition)
    println!("--- Pipe (left-to-right composition) ---");
    let transform = pipe(add_one, multiply_by_two);
    println!("pipe(add_one, multiply_by_two)(5) = {}", transform(5)); // (5+1)*2 = 12

    // Multi-step transformation
    let value = pipe(add_one, pipe(multiply_by_two, to_string))(5);
    println!("add_one -> multiply_by_two -> to_string: {}", value);

    // Using the Pipeable trait
    let transform_trait = add_one.pipe(multiply_by_two);
    let actual = transform_trait(5);
    assert_eq!(actual, 12);
    println!("add_one.pipe(multiply_by_two)(5) = {}", actual);

    // Multi-step transformation using the Pipeable trait
    let value = add_one
        .pipe(multiply_by_two)
        .pipe(divide_by_two)
        .pipe(to_string)(5);
    assert_eq!(value, "6");
    println!(
        "add_one.pipe(multiply_by_two).pipe(divide_by_two).pipe(to_string)(5) = {}",
        value
    );

    // Using compose (right-to-left composition)
    println!("\n--- Compose (right-to-left composition) ---");
    let transform_compose = compose(multiply_by_two, add_one);
    let actual = transform_compose(5);
    assert_eq!(actual, 12);
    println!("compose(multiply_by_two, add_one)(5) = {}", actual);

    // Using the Composable trait
    let transform_compose_trait = multiply_by_two.compose(add_one);
    println!(
        "multiply_by_two.compose(add_one)(5) = {}",
        transform_compose_trait(5)
    );

    // Multi-step transformation using the Composable trait
    let value = to_string
        .compose(divide_by_two)
        .compose(multiply_by_two)
        .compose(add_one)(5);
    assert_eq!(value, "6");
    println!(
        "to_string.compose(divide_by_two).compose(multiply_by_two).compose(add_one)(5) = {}",
        value
    );

    // Currying example
    println!("\n--- Currying ---");
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let curried_add = curry(add);
    let add_five = curried_add(5);
    println!("curry(add)(5)(3) = {}", add_five(3)); // 5+3 = 8

    // Using identity
    println!("\n--- Identity ---");
    println!("identity(42) = {}", identity(42));
    println!("42.identity() = {}", 42.identity());

    // Function composition with string parsing example
    println!("\n--- Complex Composition Example ---");
    fn add_one_and_stringify(x: u32) -> String {
        format!("{}", x + 1)
    }

    let parse_and_mult_2 = |x: String| x.parse::<i32>().map(|x| x * 2);
    let add_one_stringify_parse_mult_2 = pipe(add_one_and_stringify, parse_and_mult_2);

    let result = add_one_stringify_parse_mult_2(5);
    println!("Complex composition result: {:?}", result); // Ok(12)
}
