use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let number = rand::thread_rng().gen_range(0, 11);

    println!("number: {}", number);

    // if else if else

    if number < 5 {
        println!("The number is lower than 5");
    } else if number == 5 {
        println!("The number is 5");
    } else {
        println!("The number is higher than 5");
    }

    // match

    match number.cmp(&5) {
        Ordering::Less => println!("The number is lower than 5"),
        Ordering::Equal => println!("The number is equal to 5"),
        Ordering::Greater => println!("The number is greater than 5")
    };

    // if used in an expression (like ternary operator or Python)

    let odd_or_not = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number is {}", odd_or_not);
}
