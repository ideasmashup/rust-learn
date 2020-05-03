use std::io;

fn main() {
    loop {
        let mut input = String::new();
        println!("\nType a \"good\" value:");
        io::stdin().read_line(&mut input)
            .expect("Can't read your input");
    
        let _value = match input.trim().parse::<i32>() {
            Ok(v) => match v {
                0..=10 => {
                    println!("{:?} is a correct value!", v);
                },
                _ => {
                    println!("Not in 0 to 10 included range!");
                    continue;
                }
            },
            Err(e) => {
                println!("Not a 32bits integer! Error -> {:?}", e);
                continue;
            }
        };

        println!("Got a good unsigned integer within expected range, exiting happily!");
        return;
    }
}
