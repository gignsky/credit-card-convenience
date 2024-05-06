# Price Calculator in Rust

This is a simple command-line application written in Rust that calculates the final price of a product after applying various fees. The program takes the original price of a product as input and calculates the final price that should be charged to cover the costs of invoice fees, payment fees, and a fixed price.

## Constants

The program uses the following constants:

- `FIXED_PRICE`: A fixed price that is added to the original price of the product.
- `INVOICE_FEES`: The percentage of the price that is taken as invoice fees.
- `PAYMENT_FEES`: The percentage of the price that is taken as payment fees.
- `RATES`: The sum of `INVOICE_FEES` and `PAYMENT_FEES`.
- `INVERTED_RATE`: The inverse of `RATES`, used to calculate the final price.

## Functions

The program defines the following functions:

- `main`: The main function of the program. It continuously asks the user to enter the price of a product, calculates the final price, and displays the results.
- `calculate`: Takes the original price of a product as input and calculates the final price that should be charged.
- `verify`: Takes the original price and the final price as input and prints a detailed breakdown of the fees and the final price.
- `format_price`: Takes a price as input and returns a string that represents the price formatted as a dollar amount.

## How to Run

To run the program, you need to have Rust installed on your machine. You can then compile and run the program using the `cargo run` command.

