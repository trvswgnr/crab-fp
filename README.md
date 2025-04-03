# Functional Programming for Rust

This crate provides functional programming abstractions and utilities for Rust.
It implements common functional programming typeclasses and utilities that enable
functional programming patterns while maintaining Rust's strong type safety.

## Main Components

* **Core Typeclasses**: Functor, Applicative and related traits that form the
  foundation of the functional programming approach.

* **Standard Type Implementations**: Ready-to-use implementations for common Rust
  types like Option, Result, and Vec.

* **Utility Functions**: Helpers for function composition, currying, and other
  functional programming techniques.

## Examples

The project includes several examples that demonstrate different aspects of the library:

1. **Basic Usage** - Simple examples of Functor and Applicative operations:
   ```bash
   cargo run --example basic_usage
   ```

2. **Price Calculator** - Demonstrates how the same business logic can be applied across different container types:
   ```bash
   cargo run --example price_calculator
   ```

3. **Function Composition** - Shows how to use pipe, compose, curry and other function utilities:
   ```bash
   cargo run --example function_composition
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

// Use function composition
fn add_one(x: i32) -> i32 { x + 1 }
fn multiply_by_two(x: i32) -> i32 { x * 2 }
let transform = pipe(add_one, multiply_by_two);
assert_eq!(transform(5), 12);  // (5+1)*2 = 12
``` 