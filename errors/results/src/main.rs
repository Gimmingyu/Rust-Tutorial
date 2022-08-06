/**
 * 러스트가 제공하는 Result 타입은 Ok와 Err로 이루어져 있다.
 * enum Result<T, E> {
 *      Ok(T),
 *      Err(E),
 * }
 * T와 E는 제네릭 타입 파라미터이다.
 * T는 성공한 경우에 Ok variant 내에 반환될 값의 타입을 나타내고,
 * E는 실패한 경우에 Err variant 내에 반환될 에러의 타입을 나타낸다.
 * 이를 통해 복구 가능한 에러를 적절하게 다룰 수 있을 것이다.
 *
 */
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let _f = File::open("hello.txt");

    let mut _f = match _f {
        Ok(file) => file,
        // kind 메소드를 이용해서 어떤 종류의 에러인지 알 수 있다.
        // 표준 라이브러리에서 제공하는 ErrorKind를 이용해서 NotFound에러와 매치 시킨다.
        // 일치하는 경우 create를 이용해서 파일을 만든다.
        // 이렇게 match 뒤에 조건을 붙이는 것을 매치 가드라고 부른다.
        // 패턴 매칭을 더 정제해주는 용도로 쓰면 좋다.
        // 단, 이때 패턴에는 ref가 필요하다. ref가 없다면 error의 소유권이 조건문으로
        // 넘어가게 되기 때문에 참조자를 얻기위해 ref를 사용한다.
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                // 만드는 게 실패했을 경우의 패닉
                Err(e) => {
                    panic!("Tried to create file but there was an error: {}", e)
                }
            }
        }
        // 다른 이유로 Open 실패했을 경우의 패닉
        Err(error) => {
            panic!("There was an error trying to open file: {:?}", error);
        }
    };

    // unwrap과 expect
    // unwrap은 Result값이 Ok면 Ok를 반환해주고, Err이면 panic!을 호출한다.
    // expect는 panic! 에러 메세지를 선택할 수 있게 해준다.
    // 디버깅 할 때 편할 것...
    let _file = File::open("newfile.txt").unwrap();
}

// 에러가 발생하면 호출한 쪽으로 에러를 반환하는 함수.
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// ? 연산자를 이용한 숏코딩
// ? 연산자를 이용하면 에러를 호출하는 쪽으로 반환할 수 있다.
// 단, Result를 반환하는 함수에서만 사용 가능하다. 
fn version1() -> Result<String, io::Error> {
    let f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn version2() -> Result<String, io::Error> {
    let mut s = String::new();
    let f = File::open("hello.txt")?.read_to_string(mut &s)?;
    Ok(s)
}