use std::io;

fn main() {
    loop {
        println!("\nInput a temperature (e.g. 1°C or 2.3°K):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Couldn't read user input!");

        // check if user wants to exit
        if input.contains("q") {
            println!("Thanks for the run!");
            return;
        }

        // clean user input
        let mut value = String::new();
        let mut unit = '°';

        for c in input.chars() {// single-pass cleaning of all input chars
            match c {
                x if x.is_numeric() || x == '.' => {
                    value.push(c);
                },
                'C' => {
                    unit = 'C';
                },
                'K' => {
                    unit = 'K';
                }
                _ => {/* invalid char, skip */}
            }
        }

        // parse final value
        let value: f32 = match value.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Value isn't a valid numeric value (e.g. like 42 or 37.5)");
                continue;
            }
        };
        
        // do conversion
        match unit {
            'C' => {
                println!("{}°C is equal to {}°K", value, value + 273.15);
                continue;
            },
            'K' => {
                println!("{}°K is equal to {}°C", value, value - 273.15);
                continue;
            },
            _ => {
                println!("Missing or incorrect unit, try again! Or type \"quit\"");
                continue;
            }
        }                
    }
}
