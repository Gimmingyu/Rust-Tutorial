use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let _secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", _secret_number);

    loop {
        println!("Please input your guess.");
    
        // mutable 변수 guess 선언.
        // new는 String 타입 연관 합수.
        // 연관 함수는 하나의 타입을 위한 함수.
        // 새로운 빈 String 인스턴스와 연결된 가변변수를 생성한다.
        let mut guess = String::new();
        // String 타입을 위한 함수라고 생각하면 된다.
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
            
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&_secret_number) {
            Ordering::Less => println!("Too Less!"),
            Ordering::Greater => println!("Too Greater!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}
