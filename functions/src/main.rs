fn main() {
    println!("Hello, world!");

    another_function();
    new_function(5);
    simple_function_with_multiple_param(5, 10);

    let x = 6;

    let y = {
        let x = 3;
        // 세미콜론을 찍지 않음으로 표현식의 종결을 나타낸다.
        x + 1
        // y = 4가 된다.
    };

    println!("x = {}", x);
    println!("y = {}", y);

    let z = five();
    println!("z = {}", z);

    let x = plus_one(5);
    println!("x = {}", x);
}

// Simple function with return value
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// Simple function
fn another_function() {
    println!("Another function!");
}

// Simple function with single parameter
fn new_function(x: i32) {
    println!("The value of x is {}", x);
}

// Simple function with multiple parameters
fn simple_function_with_multiple_param(x: i32, y : i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}