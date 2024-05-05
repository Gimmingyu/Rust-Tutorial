// 구조체를 쓰는 것 보다 열거형을 쓰는 것이 더 간결해지는 예시
enum IpAddr {
    V4(String),
    V6(String),
}

struct IpAddrStruct {
    kind: IpAddr,
    address: String,
}

fn main() {
    let four = IpAddr::V4;
    let six = IpAddr::V6;

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // Option Test

    let some_number = Some(5);
    let some_string = Some("Hello world");

    // None을 사용할 것이라면 타입을 명시해주어야 한다.
    // let var_none: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}
