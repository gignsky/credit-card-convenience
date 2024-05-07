use std::io;

const FIXED_THIRTY_CENT_FEE: f64 = 0.30;
const MISC_STRIPE_INVOICE_FEE: f64 = 0.004;
const CREDIT_CARD_FEE_PERCENTAGE: f64 = 0.029;
const FEE_PERCENTAGE_OF_TOTAL: f64 = MISC_STRIPE_INVOICE_FEE + CREDIT_CARD_FEE_PERCENTAGE;
const INVERTED_RATE: f64 = 1.0 - FEE_PERCENTAGE_OF_TOTAL;

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
    let fixed_price = og_price + FIXED_THIRTY_CENT_FEE;
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

fn format_price(price: f64) -> String {
    //price.to_string()
    format!("${:.2}", price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        assert_eq!(format!("{:.2}", calculate(10.0)), "10.65");
        assert_eq!(format!("{:.2}",calculate(125.0)), "129.58");
        assert_eq!(format!("{:.2}",calculate(625.0)), "646.64");
    }

    #[test]
    fn test_format_price() {
        assert_eq!(format_price(10.0), "$10.00");
        assert_eq!(format_price(20.123), "$20.12");
        assert_eq!(format_price(30.999), "$31.00");
    }
}
