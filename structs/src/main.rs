struct User {
    // String으로 정의함으로써 구조체가 유효한 동안은 구조체가 username의 소유권을 가진다.
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct OtherUser {
    // 문자 슬라이스 타입, 구조체가 소유권을 가지지 않는다.
    // 참조할 수는 있지만, 라이프타임을 명시해주어야 한다.
    // 그렇지않으면 컴파일 에러 발생.
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut newUser = User {
        email: String::from("someone@email.com"),
        username: String::from("some_username"),
        active: true,
        sign_in_count: 1,
    };

    newUser.email = String::from("another_email@example.com");

    // newUser 인스턴스 재사용하기
    let mut user2 = User {
        email: String::from("someone22@email.com"),
        username: String::from("some_username22"),
        active: newUser.active,
        sign_in_count: newUser.sign_in_count,
    };

    // tuple struct 튜플 구조체 예시
    // 이름은 없고 필드마다 타입은 다르게 정의 가능하다.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // 필드가 없는 유사 유닛 구조체??
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn simple_build_user(email: String, username: String) -> User {
    // 변수명과 필드명이 같으면 축약 가능
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
