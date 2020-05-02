fn main() {
    // types

    // INTEGERS
    // integers declarations
    
    let int = 4u8;
    println!("Value: {}", int);

    let int: u8 = 4;
    println!("Value: {}", int);

    let min = u8::min_value();
    let max = u8::max_value();

    println!("Value: {} (size={}, MIN={}, MAX={})\n", int, "8bits", min, max);

    // ------

    let int = 126i8;
    println!("Value: {}", int);

    let int: i8 = 126;
    println!("Value: {}", int);

    let min = i8::min_value();
    let max = i8::max_value();

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
    

}
