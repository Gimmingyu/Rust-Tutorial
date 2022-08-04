// 구조체를 쓰는 것 보다 열거형을 쓰는 것이 더 간결해지는 예시
enum IpAddr {
    V4(String),
    V6(String),
}

// 러스트에는 null이 없지만, 값의 존재 혹은 부재를 표현할 수 있는 열거형이 있다.
// Some(T)는 어떠한 값이 있다는 것을 의미하며, None은 값이 없다는 것을 의미한다.
enum Option<T> {
    Some(T),
    None,
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
    let var_none: Option<i32> = None;
}