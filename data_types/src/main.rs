fn main() {
    let parse_test: i32 = "42".parse().expect("Not a number");
    println!("parse_test = {}", parse_test);

    // 부통 소수점 예시

    let _x = 2.0;

    let _y: f32 = 3.0; // f32 || f64

    let _sum = 5 + 10;

    let _difference = 95.5 - 4.3;

    let _product = 4 * 30;

    let _quotient = 56.7 / 32.2;

    let _remainder = 43 % 6;

    let _t = true;

    let _f: bool = false;

    let _c = 'z';

    let _z = 'Z';

    let _heart_eyed_cat = '😻';

    println!("heart_eyed_cat = {}", _heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("tup = ({}, {}, {})", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;

    println!("x = {}, y = {}, z = {}", x, y, z);

    // Rust에서 배열은 고정된 크기를 가진다.
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    // tuple은 개별 값을 밖으로 빼내올 때 패턴 매칭을 사용한 구조해체,
    // array에서는 대괄호를 이용해서 색인으로 접근.
    let first = a[0];
    let second = a[1];

    println!("first = {}, second = {}", first, second);
}
