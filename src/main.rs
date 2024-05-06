use std::io;

const FIXED_PRICE: f64 = 0.30;
const INVOICE_FEES: f64 = 0.004;
const PAYMENT_FEES: f64 = 0.029;
const RATES: f64 = INVOICE_FEES + PAYMENT_FEES;
const INVERTED_RATE: f64 = 1.0 - RATES;

fn main() {
    loop {
        println!(
            "Welcome! Please enter the price of the product in dollars and cents (e.g. 10.99):"
        );

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let price: f64 = match input.trim().parse() {
            Ok(price) => price,
            Err(_) => {
                println!("Invalid input. Please enter a valid price in dollars and cents.");
                continue;
            }
        };

        if price == 0.0 {
            break;
        }

        let charge_price = calculate(price);
        verify(price, charge_price);
    }
}

fn calculate(og_price: f64) -> f64 {
    let fixed_price = og_price + FIXED_PRICE;
    let return_price = fixed_price / INVERTED_RATE;
    return_price
}

fn verify(og_price: f64, charge_price: f64) {
    println!(
        "The original price of the product is {}",
        format_price(og_price)
    );
    println!(
        "The charge price of the product is: {}",
        format_price(charge_price)
    );
    let invoice_fee = charge_price * INVOICE_FEES;
    println!("The invoice fee is: {}", format_price(invoice_fee));
    let cc_fee = charge_price * PAYMENT_FEES + FIXED_PRICE;
    println!("The fee is: {}", format_price(cc_fee));
    let total_fee = invoice_fee + cc_fee;
    println!("The total fee is: {}", format_price(total_fee));
    let price_after_fees = charge_price - total_fee;
    println!(
        "The price after fees is: {}",
        format_price(price_after_fees)
    );

    println!("-----------------------------------");
    println!(
        "The Additional amount to charge is {}",
        format_price(charge_price - og_price)
    );
    println!("-----------------------------------");
}

fn format_price(price: f64) -> String {
    //price.to_string()
    format!("${:.2}", price)
}
