use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let mut rng = rand::thread_rng();
    let secret: u16 = rng.gen_range(0, 101);
    
    println!("The secret is {}", secret);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    let guess: u16 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess {
        0..=100 => {/* guess in expected range, do nothing */},
        _ => println!("You must pick a number between 0 and 100!"),
    }

    match guess.cmp(&secret) {
        Ordering::Less => println!("Too low!"),
        Ordering::Greater => println!("Too high!"),
        Ordering::Equal => println!("GG! Well played!"),
    }

}
