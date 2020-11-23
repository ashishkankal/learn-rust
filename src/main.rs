fn main() {
    tuple_example();
    another_function(5);
    let x = five();

    println!("The return value of function five in x is: {}", x);

    conditional_example(x);
}

fn tuple_example() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {} {} {}", x, y, z);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn conditional_example(number: i32) {
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
