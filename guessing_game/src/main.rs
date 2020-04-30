use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let mut rng = rand::thread_rng();
    let secret: u8 = rng.gen_range(0, 101);
    
    println!("The secret is {}", secret);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {}", guess);

}
