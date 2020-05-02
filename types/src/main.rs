use std::mem;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    // types

    println!("\n--- DATA TYPES ---\n");

    // INTEGERS
    // integers declarations
    
    println!("--- Integers\n");

    let int = 0;
    println!("Value: {}", int);
    println!("Specs: {} {} bytes\n", type_of(int), mem::size_of_val(&int));

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

    println!("Specs: i8 {} byte from {} to {}", mem::size_of_val(&int), min, max);

    // ------

    let int = 255u8;
    println!("Value: {}", int);

    let int: u8 = 255_u8;        // ugly but works
    println!("Value: {}", int);

    let int: u8 = 2_5_5_u8;      // even uglier but works too ;)
    println!("Value: {}", int);

    let min = u8::MIN;
    let max = u8::MAX;

    println!("Specs: u8 {} byte from {} to {}\n", mem::size_of_val(&int), min, max);

    // ------

    let int = 2048i16;
    println!("Value: {}", int);

    let int = 2_048i16;
    println!("Value: {}", int);

    let int: i16 = 2048;
    println!("Value: {}", int);

    let min = i16::MIN;
    let max = i16::MAX;

    println!("Specs: i16 {} bytes from {} to {}\n", mem::size_of_val(&int), min, max);
    
    // ------

    let int = 65535u16;
    println!("Value: {}", int);

    let int = 65_535u16;
    println!("Value: {}", int);

    let int: u16 = 65535;
    println!("Value: {}", int);

    let min = u16::MIN;
    let max = u16::MAX;

    println!("Specs: u16 {} bytes from {} to {}\n", mem::size_of_val(&int), min, max);
    
    // ------

    let int = 1_234_567_890;
    println!("Value: {}", int);

    let int = 1234567890isize;
    println!("Value: {}", int);

    let int = 1_234_567_890isize;
    println!("Value: {}", int);

    let int: isize = 1234567890;
    println!("Value: {}", int);

    let min = isize::MIN;
    let max = isize::MAX;

    println!("Specs: isize {} bytes from {} to {}\n", mem::size_of_val(&int), min, max);
    
    // ------

    let int = 1234567890;
    println!("Value: {}", int);

    let int = 1_234_567_890;
    println!("Value: {}", int);

    let int = 1234567890usize;
    println!("Value: {}", int);

    let int: usize = 1234567890;
    println!("Value: {}", int);

    let min = usize::MIN;
    let max = usize::MAX;

    println!("Specs: usize {} bytes from {} to {}\n", mem::size_of_val(&int), min, max);
    
    // ------

    let int = 1234567890;
    println!("Value: {}", int);

    let int = 1_234_567_890;
    println!("Value: {}", int);

    let int = 1234567890i64;
    println!("Value: {}", int);

    let int = 1_234_567_890i64;
    println!("Value: {}", int);

    let int: i64 = 1234567890;
    println!("Value: {}", int);

    let min = i64::MIN;
    let max = i64::MAX;

    println!("Specs: i64 {} bytes from {} to {}\n", mem::size_of_val(&int), min, max);
    
    // ------

    let int = 1234567890u64;
    println!("Value: {}", int);

    let int = 1_234_567_890u64;
    println!("Value: {}", int);

    let int: u64 = 1234567890;
    println!("Value: {}", int);

    let int: u64 = 1_234_567_890;
    println!("Value: {}", int);

    let min = u64::MIN;
    let max = u64::MAX;

    println!("Specs: u64 {} bytes from {} to {}\n", mem::size_of_val(&int), min, max);
    
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

    println!("Specs: i128 {} bytes from {} to {}\n", mem::size_of_val(&int), min, max);
    
    // ------

    let float: f64 = 12345.6789;
    println!("Value: {}", float);

    let float: f64 = 12_345.6_789;
    println!("Value: {}", float);

    let float = 12345.6789f64;
    println!("Value: {}", float);

    let float = 128345.6_789f64;
    println!("Value: {}", float);

    let min = f64::MIN;
    let max = f64::MAX;

    println!("Specs: f64 {} bytes from {} to {}\n", mem::size_of_val(&float), min, max);
        
}
