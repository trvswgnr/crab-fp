# crab-fp

This crate provides functional programming abstractions and utilities for Rust.
It implements common functional programming typeclasses and utilities that enable
functional programming patterns while maintaining Rust's strong type safety.

## Main Components

* **Core Typeclasses**: Functor, Applicative, Monad, and related traits that form the
  foundation of the functional programming approach.

* **Standard Type Implementations**: Ready-to-use implementations for common Rust
  types like Option, Result, and Vec.

* **Utility Functions**: Helpers for function composition, currying, pipelines, and other
  functional programming techniques.

## Features

- **Type-Safe Functional Patterns**: Use functional programming patterns with Rust's strict type system
- **Consistent Interface**: Apply the same operations across different container types
- **Function Composition**: Combine functions with pipe and compose operations
- **Currying Support**: Transform multi-parameter functions into chains of single-parameter functions
- **Typeclass Hierarchy**: Full implementation of Functor → Applicative → Monad hierarchy

## Examples

The project includes several examples that demonstrate different aspects of the library:

1. **Basic Usage** - Simple examples of Functor, Applicative, and Monad operations:
   ```bash
   cargo run --example basic_usage
   ```

2. **Price Calculator** - Demonstrates how the same business logic can be applied across different container types:
   ```bash
   cargo run --example price_calculator
   ```

3. **Function Composition** - Shows how to use pipe, compose, curry and other function utilities:
   ```bash
   cargo run --example utils
   ```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
crab-fp = { git = "https://github.com/trvswgnr/crab-fp" }
```

Then import everything and use:

```rust
use crab_fp::*;

// Map over any Functor
let opt = Some(5).fmap(|x| x * 2);  // Some(10)
let vec = vec![1, 2, 3].fmap(|x| x * 2);  // [2, 4, 6]

// Apply functions contained in a context to values in a context
let opt_val = Some(5);
let opt_fn = Some(|x: i32| x + 3);
let applied = opt_val.apply(opt_fn);  // Some(8)

// Use Monad bind operations
let opt = Some(5).bind(|x| Some(x * 2));  // Some(10)

// Use function composition
fn add_one(x: i32) -> i32 { x + 1 }
fn multiply_by_two(x: i32) -> i32 { x * 2 }

// Compose (right to left)
let transform = compose(multiply_by_two, add_one);
assert_eq!(transform(5), 12);  // (5+1)*2 = 12

// Pipe (left to right)
let transform = pipe(add_one, multiply_by_two);
assert_eq!(transform(5), 12);  // (5+1)*2 = 12

// Function currying
fn add(a: i32, b: i32) -> i32 { a + b }
let curried_add = curry(add);
let add_five = curried_add(5);
assert_eq!(add_five(3), 8);  // 5+3 = 8
``` 