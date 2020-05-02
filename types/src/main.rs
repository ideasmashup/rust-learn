use std::mem;

fn main() {
    // types

    // INTEGERS
    // integers declarations
    
    println!("\n--- DATA TYPES ---\n");

    let int = 127i8;
    println!("Value: {}", int);

    let int: i8 = 127;
    println!("Value: {}", int);

    let min = i8::MIN;
    let max = i8::MAX;

    /*
       let min = i8::min_value();
       let max = i8::max_value();

       // they work too but don't exist for float types (dunno why)
    */

    println!("Specs: {} byte from {} to {}\n", mem::size_of_val(&int), min, max);

    // ------

    let int = 255u8;
    println!("Value: {}", int);

    let int: u8 = 255;
    println!("Value: {}", int);

    let min = u8::MIN;
    let max = u8::MAX;

    println!("Specs: {} byte from {} to {}\n", mem::size_of_val(&int), min, max);

    // ------

    let int = 2048;
    println!("Value: {}", int);

    let int = 2_048;
    println!("Value: {}", int);

    let int = 2048i16;
    println!("Value: {}", int);

    let int: i16 = 2048;
    println!("Value: {}", int);

    let min = u16::MIN;
    let max = u16::MAX;

    println!("Specs: {} bytes from {} to {}\n", mem::size_of_val(&int), min, max);
    
    // ------

    let int = 65535;
    println!("Value: {}", int);

    let int = 65_535;
    println!("Value: {}", int);

    let int = 65535u16;
    println!("Value: {}", int);

    let int: u16 = 65535;
    println!("Value: {}", int);


    println!("Specs: {} bytes from {} to {}\n", mem::size_of_val(&int), min, max);
    
    // ------

    let int = 1234567890;
    println!("Value: {}", int);

    let int = 1_234_567_890;
    println!("Value: {}", int);

    let int: isize = 1234567890;
    println!("Value: {}", int);

    let min = isize::min_value();
    let max = isize::max_value();

    println!("Specs: {} bytes from {} to {}\n", mem::size_of_val(&int), min, max);
    
    // ------

    let int = 1234567890;
    println!("Value: {}", int);

    let int = 1_234_567_890;
    println!("Value: {}", int);

    let int = 1234567890u64;
    println!("Value: {}", int);

    let int: usize = 1234567890;
    println!("Value: {}", int);

    let min = usize::MIN;
    let max = usize::MAX;

    println!("Specs: {} bytes from {} to {}\n", mem::size_of_val(&int), min, max);
    
    // ------

    let int: i128 = 170141183460469231731687303715884105727;
    println!("Value: {}", int);

    let int: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    println!("Value: {}", int);

    let int = 170141183460469231731687303715884105727i128;
    println!("Value: {}", int);

    let int = 170_141_183_460_469_231_731_687_303_715_884_105_727i128;
    println!("Value: {}", int);

    let min = i128::MIN;
    let max = i128::MAX;

    println!("Specs: {} bytes from {} to {}\n", mem::size_of_val(&int), min, max);
        
    
}
