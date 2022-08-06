fn main() {
    let mut s = String::from("foo");

    println!("s = {}", s);

    s.push_str("bar");

    println!("s = {}", s);

    let mut s2 = "foobar".to_string();
    let foobar = "foobar".to_string();

    println!("s2 = {}", s2);
    s2.push_str(&foobar);
    println!("s2 = {}", s2);
    // 소유권이 아닌 참조자를 보내야한다.
    s.push_str(&s2);
    println!("s = {}", s);
    s.push('l');
    println!("s = {}", s);

    let s1 = String::from("hello, ");
    let s2 = String::from("world!");

    // s1은 더이상 사용 불가능하다.
    let s3 = s1 + &s2;
    println!("s3 = {}", s3);

    let s1 = String::from("hello, ");
    let s2 = String::from("world!");

    {
        let mut copy = s1.to_string();
        copy.push_str(&s2);
        println!("copy = {}", copy);
        println!("s2 = {}", s2);
    }
    println!("s1 = {}", s1);
    let s3 = s1 + &s2;
    println!("s3 = {}", s3);
    println!("s2 = {}", s2);

    let s1 = String::from("tik");
    let s2 = String::from("tak");
    let s3 = String::from("toe");

    let sentence = s1 + "-" + &s2 + "-" + &s3;
    println!("sentence = {}", sentence);
    // s2, s3는 사용가능, (borrow 했으니까) s1은 사용 불가능.
    println!("s2 = {}", s2);
    println!("s3 = {}", s3);

    let s1 = String::from("tik");
    let s2 = String::from("tak");
    let s3 = String::from("toe");

    // format을 이용하면 소유권도 가져가지 않는다.
    let sentence = format!("{}-{}-{}", s1, s2, s3);
    println!("sentence = {}", sentence);
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
    println!("s3 = {}", s3);

    let s = String::from("Hello");

    // let h = s[0];
    // println!("h = {}", h);
    // Error ! String은 슬라이싱을 지원하지 않는다.

    // String은 내부적으로 벡터의 구조를 가진다.
    let len = s.len();
    println!("len = {}", len); // len = 5
    // "Hello"를 저장하고 있는 Vec가 5바이트 길이라는 뜻

    let hello = "Здравствуйте";
    let len = hello.len();
    println!("len = {}", len); // len = 24...
    // 12가 아닌 24가 나오는 이유는 hello를 utf-8로 인코딩한 바이트들의 크기가 24이기 떄문이다.
    // 각각의 유니코드 스칼라 값이 2바이트를 차지해서 나오는 결과...
    // 같은 이유로 아래의 코드는 기대치 않은 값을 반환할 것이다. 따라서 러스트는 컴파일 에러를 일으킨다.

    // Invalid Example
    // let answer = &hello[0];
    // println!("answer = {}", answer);

    // Valid Example
    let answer = &hello[0..4]; // 4바이트만큼 담은 슬라이싱
    println!("answer = {}", answer);

    // let answer = &hello[0..1]; ==> Error !

    for i in hello.chars() {
        println!("{}", i);
    }
}