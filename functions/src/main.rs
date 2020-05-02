fn main() {
    // functions experiments
    my_function();

    f2(42, "meaning of life");

    fn local_function() {
        println!("Can this work ?");
    }

    local_function();

    // expressions experiments
    let value = "Wednesday";
    println!("value = {}", value);

    let value = "Shadow";
    println!("value = {}", value);

    let value = concat!("Shadow", " ", "Moon");
    println!("value = {}", value);

    let value = concat!("Shadow", " ", concat!("Moon"));
    println!("value = {}", value);

    let value = format!("Shadow {}", "Moon");
    println!("value = {}", value);

    let value = 1;
    let value = {
        println!("one plus ?");
        value + 1
    };
    println!("value = {:?}", value);

    // BE CAREFUL
    // works!
    let x = {
        let y = 5;
        y + 5
    };
    println!("value = {}", x);

    // doesn't work the same!
    let x = {
        let y = 5;
        y + 5; // trailing ';' here messes up the expression value of whole {...} scope
    };
    println!("value = {:?}", x);
}

fn my_function() {
    println!("Hello, nobody.");
}

fn f2(arg1: i32, arg2: &str) {
    println!("Printing {} and {}", arg1, arg2);
}