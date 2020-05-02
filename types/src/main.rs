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

    let value = 0;
    println!("Value: {}", value);
    println!("Specs: {} {} bytes -- default inference\n", type_of(value), mem::size_of_val(&value));

    let value = 127i8;
    println!("Value: {}", value);

    let value: i8 = 127;
    println!("Value: {}", value);

    let min = i8::MIN;
    let max = i8::MAX;

    /*
       let min = i8::min_value();
       let max = i8::max_value();

       // they work too but don't exist for value types (dunno why)
    */

    println!("Specs: i8 {} byte from {} to {}\n", mem::size_of_val(&value), min, max);

    // ------

    let value = 255u8;
    println!("Value: {}", value);

    let value: u8 = 255_u8;        // ugly but works
    println!("Value: {}", value);

    let value: u8 = 2_5_5_u8;      // even uglier but works too ;)
    println!("Value: {}", value);

    let min = u8::MIN;
    let max = u8::MAX;

    println!("Specs: u8 {} byte from {} to {}\n", mem::size_of_val(&value), min, max);

    // ------

    let value = 2048i16;
    println!("Value: {}", value);

    let value = 2_048i16;
    println!("Value: {}", value);

    let value: i16 = 2048;
    println!("Value: {}", value);

    let min = i16::MIN;
    let max = i16::MAX;

    println!("Specs: i16 {} bytes from {} to {}\n", mem::size_of_val(&value), min, max);
    
    // ------

    let value = 65535u16;
    println!("Value: {}", value);

    let value = 65_535u16;
    println!("Value: {}", value);

    let value: u16 = 65535;
    println!("Value: {}", value);

    let min = u16::MIN;
    let max = u16::MAX;

    println!("Specs: u16 {} bytes from {} to {}\n", mem::size_of_val(&value), min, max);
    
    // ------

    let value = 1_234_567_890;
    println!("Value: {}", value);

    let value = 1234567890isize;
    println!("Value: {}", value);

    let value = 1_234_567_890isize;
    println!("Value: {}", value);

    let value: isize = 1234567890;
    println!("Value: {}", value);

    let min = isize::MIN;
    let max = isize::MAX;

    println!("Specs: isize {} bytes from {} to {}\n", mem::size_of_val(&value), min, max);
    
    // ------

    let value = 1234567890;
    println!("Value: {}", value);

    let value = 1_234_567_890;
    println!("Value: {}", value);

    let value = 1234567890usize;
    println!("Value: {}", value);

    let value: usize = 1234567890;
    println!("Value: {}", value);

    let min = usize::MIN;
    let max = usize::MAX;

    println!("Specs: usize {} bytes from {} to {}\n", mem::size_of_val(&value), min, max);
    
    // ------

    let value = 1234567890;
    println!("Value: {}", value);

    let value = 1_234_567_890;
    println!("Value: {}", value);

    let value = 1234567890i64;
    println!("Value: {}", value);

    let value = 1_234_567_890i64;
    println!("Value: {}", value);

    let value: i64 = 1234567890;
    println!("Value: {}", value);

    let min = i64::MIN;
    let max = i64::MAX;

    println!("Specs: i64 {} bytes from {} to {}\n", mem::size_of_val(&value), min, max);
    
    // ------

    let value = 1234567890u64;
    println!("Value: {}", value);

    let value = 1_234_567_890u64;
    println!("Value: {}", value);

    let value: u64 = 1234567890;
    println!("Value: {}", value);

    let value: u64 = 1_234_567_890;
    println!("Value: {}", value);

    let min = u64::MIN;
    let max = u64::MAX;

    println!("Specs: u64 {} bytes from {} to {}\n", mem::size_of_val(&value), min, max);
    
    // ------

    let value: i128 = 170141183460469231731687303715884105727;
    println!("Value: {}", value);

    let value: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    println!("Value: {}", value);

    let value = 170141183460469231731687303715884105727i128;
    println!("Value: {}", value);

    let value = 170_141_183_460_469_231_731_687_303_715_884_105_727i128;
    println!("Value: {}", value);

    let min = i128::MIN;
    let max = i128::MAX;

    println!("Specs: i128 {} bytes from {} to {}\n", mem::size_of_val(&value), min, max);
    
    // -----

    // FLOATS
    // values declarations
    
    println!("--- Floats\n");

    let value = 1.0;
    println!("Value: {}", value);
    println!("Specs: {} {} bytes -- default inference\n", type_of(value), mem::size_of_val(&value));

    // -----
    
    let value: f32 = 1.234_567_89;
    println!("Value: {}", value);

    let value: f32 = 1.234_567_89;
    println!("Value: {}", value);

    let value = 1.23456789f32;
    println!("Value: {}", value);

    let value = 1.234_567_89f32;
    println!("Value: {}", value);

    let min = f32::MIN;
    let max = f32::MAX;

    println!("Specs: f32 {} bytes from {} to {}\n", mem::size_of_val(&value), min, max);

    // -----

    let value: f64 = 1.23456789;
    println!("Value: {}", value);

    let value: f64 = 1.234_567_89;
    println!("Value: {}", value);

    let value = 1.23456789f64;
    println!("Value: {}", value);

    let value = 1.234_567_89f64;
    println!("Value: {}", value);

    let min = f64::MIN;
    let max = f64::MAX;

    println!("Specs: f64 {} bytes from {} to {}\n", mem::size_of_val(&value), min, max);
    
    // -----

    // BOOLEANS
    // value declarations
    
    println!("--- Booleans\n");

    let value = true;
    println!("Value: {}", value);
    println!("Specs: {} {} bytes\n", type_of(value), mem::size_of_val(&value));

    let value = false;
    println!("Value: {}", value);
    println!("Specs: {} {} bytes\n", type_of(value), mem::size_of_val(&value));

    let value: bool = true || false && true;
    println!("Value: {}", value);
    println!("Specs: bool {} bytes\n", mem::size_of_val(&value));

    // -----

    // CHARS
    // values declarations
    
    println!("--- Characters\n");

    let value = 'a';
    println!("Value: {}", value);
    println!("Specs: {} {} bytes\n", type_of(value), mem::size_of_val(&value));

    let value = 'A';
    println!("Value: {}", value);
    println!("Specs: {} {} bytes\n", type_of(value), mem::size_of_val(&value));

    let value: char = 'à';
    println!("Value: {}", value);
    println!("Specs: char {} bytes\n", mem::size_of_val(&value));

    let value: char = '하';
    println!("Value: {}", value);
    println!("Specs: char {} bytes\n", mem::size_of_val(&value));
    
    let value: char = '😻';
    println!("Value: {}", value);
    println!("Specs: char {} bytes\n", mem::size_of_val(&value));

    // -----

    println!("--- COMPOUNDS\n");

    // -----

    // TUPLES
    // tuples declarations
    
    println!("--- Tuples\n");

    let value = (10, 20.00, "30");
    println!("Value: {:?}", value);
    println!("Specs: {} {} bytes -- default inference\n", type_of(value), mem::size_of_val(&value));

    let value: (u8, f32, &str) = (11, 22.22, "33");
    println!("Value: {:?}", value);
    println!("Specs: {} {} bytes\n", type_of(value), mem::size_of_val(&value));

    let (a, b, c) = value;
    println!("Values: {}, {}, {}", a, b, c);
    println!("Values: {}, {}, {}", value.0, value.1, value.2);

    // -----

    // ARRAYS
    // arrays declarations
    
    println!("--- Arrays\n");

    let value = [0, 1, 2, 3, 4];
    println!("Value: {:?}", value);
    println!("Specs: {} {} bytes -- default inference\n", type_of(value), mem::size_of_val(&value));

    let value: [i8; 5] = [0, 1, 2, 3, 4];
    println!("Value: {:?}", value);
    println!("Specs: {} {} bytes\n", type_of(value), mem::size_of_val(&value));

    // -----
}
