fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let x = x; // shadows the variable to become unmodifiable by others

    // shadowing reduces variables names variants for processing/transforms
    let age_str = "28";
    let age_num = age_str.parse::<u8>();

    // becomes...
    let age = "28";
    let age = age.parse::<u8>();
}
