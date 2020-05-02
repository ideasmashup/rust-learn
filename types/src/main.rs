use std::mem;

fn main() {
    // types

    // INTEGERS
    // integers declarations
    
    let int = 127i8;
    println!("Value: {}", int);

    let int: i8 = 127;
    println!("Value: {}", int);

    let min = i8::min_value();
    let max = i8::max_value();

    println!("Value: {} (size={}, MIN={}, MAX={})\n", int, "8bits", min, max);

    // ------

    let int = 255u8;
    println!("Value: {}", int);

    let int: u8 = 255;
    println!("Value: {}", int);

    let min = u8::min_value();
    let max = u8::max_value();

    println!("Value: {} (size={}, MIN={}, MAX={})\n", int, "8bits", min, max);

    // ------

    let int = 2048;
    println!("Value: {}", int);

    let int = 2_048;
    println!("Value: {}", int);

    let int = 2048i16;
    println!("Value: {}", int);

    let int: i16 = 2048;
    println!("Value: {}", int);

    let min = i16::min_value();
    let max = i16::max_value();

    println!("Value: {} (size={}, MIN={}, MAX={})\n", int, "16bits", min, max);
    
    // ------

    let int = 65535;
    println!("Value: {}", int);

    let int = 65_535;
    println!("Value: {}", int);

    let int = 65535u16;
    println!("Value: {}", int);

    let int: u16 = 65535;
    println!("Value: {}", int);

    let min = u16::min_value();
    let max = u16::max_value();

    println!("Value: {} (size={}, MIN={}, MAX={})\n", int, "16bits", min, max);
    
    // ------

    let int = 1234567890;
    println!("Value: {}", int);

    let int = 1_234_567_890;
    println!("Value: {}", int);

    let int: isize = 1234567890;
    println!("Value: {}", int);

    let min = isize::min_value();
    let max = isize::max_value();

    println!("Value: {} (size={}bits, MIN={}, MAX={})\n", int, mem::size_of::<isize>(), min, max);
    
    // ------

    let int = 1234567890;
    println!("Value: {}", int);

    let int = 1_234_567_890;
    println!("Value: {}", int);

    let int = 1234567890u64;
    println!("Value: {}", int);

    let int: usize = 1234567890;
    println!("Value: {}", int);

    let min = usize::min_value();
    let max = usize::max_value();

    println!("Value: {} (size={}bits, MIN={}, MAX={})\n", int, mem::size_of::<usize>(), min, max);
    
    // ------

    let int: i128 = 170141183460469231731687303715884105727;
    println!("Value: {}", int);

    let int: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    println!("Value: {}", int);

    let int = 170141183460469231731687303715884105727i128;
    println!("Value: {}", int);

    let int = 170_141_183_460_469_231_731_687_303_715_884_105_727i128;
    println!("Value: {}", int);

    let min = i128::min_value();
    let max = i128::max_value();

    println!("Value: {} (size={}, MIN={}, MAX={})\n", int, "128bits", min, max);
        
    
}
