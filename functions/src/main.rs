fn main() {
    // functions experiments
    my_function();

    f2(42, "meaning of life");

    fn local_function() {
        println!("Can this work ?");
    }

    local_function();

}

fn my_function() {
    println!("Hello, nobody.");
}

fn f2(arg1: i32, arg2: &str) {
    println!("Printing {} and {}", arg1, arg2);
}