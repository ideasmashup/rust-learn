use std::io;
use rand::Rng;
use std::cmp::Ordering;

const MIN_VALUE: u32 = 0;   // min inclusive value
const MAX_VALUE: u32 = 100; // max inclusive value

fn main() {
    println!("Guess the number!");

    let mut rng = rand::thread_rng();
    let secret = rng.gen_range(MIN_VALUE, MAX_VALUE + 1);
    
    println!("The secret is {}", secret);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    let err_str = format!("Please type a number between {} and {}!", MIN_VALUE, MAX_VALUE);
    let guess: u32 = guess.trim().parse().expect(&err_str);

    println!("You guessed: {}", guess);

    match guess {
        MIN_VALUE..=MAX_VALUE => {/* guess in expected range, do nothing */},
        _ => println!("You must pick a number between {} and {}!", MIN_VALUE, MAX_VALUE),
    }

    match guess.cmp(&secret) {
        Ordering::Less => println!("Too low!"),
        Ordering::Greater => println!("Too high!"),
        Ordering::Equal => println!("GG! Well played!"),
    }

}
