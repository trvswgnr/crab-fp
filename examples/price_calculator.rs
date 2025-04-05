use crab_fp::*;

type Error = Box<dyn std::error::Error>;

/// Calculates the price of an item with business logic.
///
/// Takes a price in cents, doubles it, and adds a 15% tax.
fn calculate_price(num: u32) -> u32 {
    ((num * 2) as f64 * 1.15) as u32 // doubled + tax
}

/// Formats a price in cents as a currency string.
fn format_price(num: u32) -> String {
    format!("${}.{:02}", num / 100, num % 100)
}

/// Processes price data regardless of its container type.
///
/// This demonstrates how the same business logic can be applied
/// uniformly across different container types (Vec, Option, Result, etc.).
fn process_data<F: Functor<u32>>(data: F) -> Apply1<F::Kind1, String> {
    data.fmap(calculate_price).fmap(format_price)
}

fn main() {
    println!("=== Price Calculator Example ===");
    println!("This example demonstrates how functional programming abstractions");
    println!("allow the same logic to be applied across different container types.\n");

    // Price data in cents from different sources
    let db_prices = vec![1095, 2350, 599]; // prices from database (in cents)
    let api_response: Option<u32> = Some(1499); // price from API (might be missing)
    let file_import: Result<u32, Error> = Ok(899); // price from file import (might fail)
    let failed_import: Result<u32, Error> = Err("File not found".into()); // failed import

    // Apply same business logic to different container types
    let formatted_db_prices = process_data(db_prices);
    let formatted_api_price = process_data(api_response);
    let formatted_import_price = process_data(file_import);
    let formatted_failed = process_data(failed_import);

    // Display results
    println!("Database prices: {:?}", formatted_db_prices);
    println!("API price: {:?}", formatted_api_price);
    println!("Successful import price: {:?}", formatted_import_price);
    println!("Failed import result: {:?}", formatted_failed);
} 