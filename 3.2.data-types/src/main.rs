fn main() {
    // use : to annotate a variable
    let number: i32 = "42".parse::<i32>().expect("Not a number!");
    println!("i32 number: {}", number);

    // use `_` as visual separator
    let big_number = 2_000_000;
    println!("big number: {}", big_number);

    // Numeric operations

    // addition
    let sum = 5 + 10;
    println!("5 + 10 = {}", sum);

    // substraction
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("4 * 30 = {}", product);

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    println!("56.7 / 32.2 = {}", quotient);
    println!("2 / 3 = {}", floored);

    // remainder
    let remainder = 43 % 5;
    println!("43 % 5 = {}", remainder);
}
