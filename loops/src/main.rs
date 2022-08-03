fn main() {
    // 무한루프
    // eternal_loop();

    // while문 활용해보기 
    println!("loop with condition");
    loop_with_condition();

    println!("loop_with_condition2");
    loop_with_condition2();

    println!("loop_with_condition3");
    loop_with_condition3();

    println!("loop_using_for");
    loop_using_for();
}

fn eternal_loop() {
    loop {
        println!("again!");
    }
}

fn loop_with_condition() {
    let mut number = 3;

    while number != 0 {
        println!("number: {}", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn loop_with_condition2() {
    let arr = [10, 20, 30, 40, 50];
    
    let mut index = 0;
    
    while index < 5 {
        println!("the arr[{}] is {}", index, arr[index]);
        index += 1;
    }

    println!("LIFTOFF!!!");
}

fn loop_with_condition3() {
    let arr = [10, 20, 30, 40, 50];
    
    for element in arr.iter() {
        println!("the value is {}", element);
    }

    println!("LIFTOFF!!!");
}

fn loop_using_for() {
    for number in (1..4).rev() {
        println!("number: {}", number);
    }
    println!("LIFTOFF!!!");
}