use std::io;

///This is the fixed 30 cent fee that Stripe charges on top of the below 2.9% fee for every transaction
const FIXED_THIRTY_CENT_FEE: f64 = 0.30;
///This is the 2.9% fee that Stripe charges for every transaction
const CREDIT_CARD_FEE_PERCENTAGE: f64 = 0.029;
///This is the 0.4% fee that Stripe charges for every invoice creation
const MISC_STRIPE_INVOICE_FEE: f64 = 0.004;
///This is the total percentage of the the total cost that stripe will charge before the 30 cent additional fee
const FEE_PERCENTAGE_OF_TOTAL: f64 = MISC_STRIPE_INVOICE_FEE + CREDIT_CARD_FEE_PERCENTAGE;
///This is the inverted rate of the total percentage of the total cost that stripe will charge before the 30 cent additional fee
const INVERTED_RATE: f64 = 1.0 - FEE_PERCENTAGE_OF_TOTAL;

/// Main function that runs the program.
fn main() {
    loop {
        println!("-----------------------------------");
        println!(
            "Welcome! Please enter the price of the product in dollars and cents (e.g. 10.99) | Enter Zero (0) to QUIT:"
        );
        println!("-----------------------------------");

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

        println!("-----------------------------------");
        let charge_price = calculate(price);
        verify(price, charge_price);
        println!("-----------------------------------");
    }
}

/// Calculates the charge price based on the original price.
/// Returns the calculated charge price.
fn calculate(og_price: f64) -> f64 {
    let fixed_price = og_price + FIXED_THIRTY_CENT_FEE;
    let return_price = fixed_price / INVERTED_RATE;
    return_price
}

/// Verifies the original price and the charge price.
/// Prints the original price, charge price, invoice fee, fee, total fee, and price after fees.
fn verify(og_price: f64, charge_price: f64) {
    println!(
        "The original price of the product is {}",
        format_price(og_price)
    );
    println!(
        "The charge price of the product is: {}",
        format_price(charge_price)
    );
    let invoice_fee = charge_price * MISC_STRIPE_INVOICE_FEE;
    println!("The invoice fee is: {}", format_price(invoice_fee));
    let cc_fee = charge_price * CREDIT_CARD_FEE_PERCENTAGE + FIXED_THIRTY_CENT_FEE;
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

/// Formats the price as a string with a dollar sign and two decimal places.
fn format_price(price: f64) -> String {
    format!("${:.2}", price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        assert_eq!(format!("{:.2}", calculate(10.0)), "10.65");
        assert_eq!(format!("{:.2}", calculate(125.0)), "129.58");
        assert_eq!(format!("{:.2}", calculate(625.0)), "646.64");
    }

    #[test]
    fn test_format_price() {
        assert_eq!(format_price(10.0), "$10.00");
        assert_eq!(format_price(20.123), "$20.12");
        assert_eq!(format_price(30.999), "$31.00");
    }
}
