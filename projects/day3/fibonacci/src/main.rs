use std::io;

fn fibonacci(n: u128) -> u128 {
    let mut prev: u128 = 0;
    let mut next: u128 = 1;
    let mut current: u128 = 1;

    if n < 1 {
        return 0
    }
    else if n == 1 {
        return 1;
    }
    else {
        for _i in 2..=n {
            current = prev + next;
            prev = next;
            next = current;
        }
    }
    
    current
}

fn main() {
    println!("Nth Fibonacci term solver");

    loop {
        println!("\nEnter N:");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Can't read input!");

        if number.contains("q") {
            println!("Thanks for trying this program!");
            return
        }

        let number = match number.trim().parse::<u128>() {
            Ok(num) => num,
            Err(e) => {
                println!("Incorrect value. Must be a positive integer. ({})", e);
                println!("To exit type \"q\" or \"quit\"");
                continue;
            }
        };

        let term = fibonacci(number);
        match number {
            x if x == 0 || x == 1 => {
                println!("The 1st term of the Fibonnaci sequence is {}", term);
            },
            2 => {
                println!("The 2nd term of the Fibonnaci sequence is {}", term);
            },
            3 => {
                println!("The 3rd term of the Fibonnaci sequence is {}", term);
            }
            _ => {
                println!("The {}th term of the Fibonnaci sequence is {}", number, term);
            }
        }

        // warning about overflow (this bug can only be detected as "overflow" in debug compiling in --release mode values are silently wrapped :/)
        if number > 185 {
            println!("\nwarning: 128bits unsigned integers max value is {}. As a result fibonacci terms above the 185th cause overflows thus computed values get wrapped thus becoming incorrect. Feel free to fork this project on github with num_bigint or your own prefered crate/method! ;)", u128::MAX);
        }

    }
}
