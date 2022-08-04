// 사각형의 넓이를 계산해주는 프로그램.
// 길이와 너비를 입력 받는다.
use std::io;
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn getArea(rect: Rectangle) -> u32 {
    rect.width * rect.height
}

fn main() {
    loop {
        let mut width = String::new();
        let mut height = String::new();

        println!("Pleas input width");
        io::stdin().read_line(&mut width).expect("Failed to read line");
        let width: u32 = match width.trim().parse() {
            Ok(num) => num,
            Err(_err) => {
                continue;
            }
        };

        println!("Pleas input height");
        io::stdin().read_line(&mut height).expect("Failed to read line");
        let height: u32 = match height.trim().parse() {
            Ok(num) => num,
            Err(_err) => {
                continue;
            }
        };

        let rect = Rectangle {
            width,
            height,
        };

        let area: u32 = height * width;

        println!("Area : {}", area);
    }
}