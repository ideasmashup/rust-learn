use rand::Rng;

fn main() {
    let number = rand::thread_rng().gen_range(0, 11);

    println!("number: {}", number);

    if number < 5 {
        println!("The number is lower than 5");
    } else if number == 5 {
        println!("The number is 5");
    } else {
        println!("The number is higher than 5");
    }
}
