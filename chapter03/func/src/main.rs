fn main() {
    println!("Hello, world");

    another_function();
    another_function_num(5);
    print_labeled_measurement(5, 'c');

    let x = five();
    println!("The value of x is: {}", x);

    let added_one = plus_one(5);
    println!("The value of added_one is: {}", added_one);
}

fn another_function() {
    println!("Anothre function");
}

fn another_function_num(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {} {}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
