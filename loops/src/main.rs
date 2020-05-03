use std::time::{Duration, Instant};

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

    // benchmarks
    println!("\nBenchmark comparison\n");

    // There are variations, yet I don't know why?!
    
    const RUNS:usize = 1000;
    let afast: [u8;RUNS] = [0;RUNS];

    let mut now: Instant;
    let mut turn: usize;
    let mut duration: u128;

    turn = 0;
    now = Instant::now();
    loop {
        turn += 1;
        if turn == RUNS {
            break;
        }
    }
    duration = now.elapsed().as_nanos();
    println!("{}ns to iterate 1000 times with loop...", duration);

    turn = 0;
    now = Instant::now();
    loop {
        turn += 1;
        if turn == RUNS {
            break;
        }
    }
    duration = now.elapsed().as_nanos();
    println!("{}ns to iterate 1000 times with loop...", duration);

    turn = 0;
    now = Instant::now();
    loop {
        if turn < RUNS {
           turn += 1;
        } else {
            break;
        }
    }
    duration = now.elapsed().as_nanos();
    println!("{}ns to iterate 1000 times with loop...", duration);

    turn = 0;
    now = Instant::now();
    loop {
        if turn < RUNS && afast[turn] == 0 {
            turn += 1;
        } else {
            break;
        }
    }
    duration = now.elapsed().as_nanos();
    println!("{}ns to iterate 1000 times with loop in array...", duration);

    turn = 0;
    now = Instant::now();
    while turn < RUNS {
        turn += 1;
    }
    duration = now.elapsed().as_nanos();
    println!("{}ns to iterate 1000 times with while...", duration);

    turn = 0;
    now = Instant::now();
    while turn < RUNS && afast[turn] == 0 {
        turn += 1;
    }
    duration = now.elapsed().as_nanos();
    println!("{}ns to iterate 1000 times with while in array...", duration);

    turn = 0;
    now = Instant::now();
    for num in (0..RUNS) {
        // go fast?!
    }
    duration = now.elapsed().as_nanos();
    println!("{}ns to iterate 1000 times with for x in range...", duration);

    turn = 0;
    now = Instant::now();
    for item in afast.iter() {
        // go faster?!
    }
    duration = now.elapsed().as_nanos();
    println!("{}ns to iterate 1000 times with for x in iter (u8 array)...", duration);


}
