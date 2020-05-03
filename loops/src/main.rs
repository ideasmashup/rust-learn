fn main() {
    
    // loop

    let mut counter = 0;
    
    let result = loop {
        counter += 1;

        println!("The counter is at {}", counter);

        if counter == 10 {
            break counter * 2; // why...? trailing ;
        }
    };

    println!("The result is {}", result);

    // while

    let mut number = 3;

    while number != 0 {
        println!("{} !", number);
        number -= 1;
    }

    println!("IGNITION !!!\n");

    // while "for-like"

    let arr = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut index = 0;

    while index < arr.len() {
        println!("value is {}", arr[index]);
        index += 1;
    }

    // for

    let array = [1, 2, 3, 4, 5, 6, 7, 8];

    for value in array.iter() {
        println!("value is {}", value);
    }

    for number in (0..10).rev() {
        println!("{}!", number);
    }

}
