# Trait

트레잇은 제네릭과는 다른 종류의 추상화를 돕는다.

제네릭이 타입에 대한 추상화를 제공했다면,

트레잇은 타입들이 공통적으로 갖는 동작에 대하여 추상화를 돕는다.

우리가 제네릭 타입 파라미터를 사용하는 상황에서는,

컴파일 타임에 해당 제네릭 타입이 어떤 트레잇을 구현한 타입임을 명시해야 우리가 원하는 동작을 갖도록 할 수 있다.

트레잇은 다른 언어들에서 인터페이스 라고 부르는 기능과 유사하지만, 조금 다르다.

## 트레잇 정의

어떤 타입의 동작은 우리가 해당 타입 상에서 호출할 수 있는 메소드들로 구성되어 있다.

만약 우리가 서로 다른 타입에 대해 동일한 메소드를 호출할 수 있다면 해당 타입들은 동일한 동작을 공유한다.

트레잇의 정의는 어떠한 목적을 달성하기 위해 필요한 동작의 집합을 정의하고자 메소드 시그니처들을 함께 묶는 방법이다.

```rust
pub trait Summarizable {
    fn summary(&self) -> String;
}

// 가시성 트레잇 트레잇이름 {
//     함수(이 트레잇을 구현하는 타입들이 가질 동작) 정의...
// }
```
**summary 메소드에 의해 제공되는 동작으로 구성된 Summarizable 트레잇 정의**

String 시그니처를 사용하면서 뒤에 세미콜론을 붙였다.

이로써 이 트레잇을 구현하는 각 타입은 summary 메소드에 대한 해당 타입 고유의 커스텀 동작을 제공하면서,

summary의 시그니처를 정확히 동일하게 정의하도록 강제할 수 있다.

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: String,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summarizable {
    fn summary(&self) -> String;
}
```

### 트레잇 바운드 

제네릭 타입을 공부하면서 사용했던 largest 함수.

```rust 
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    return largest
}
```

이것을 컴파일하면 PartialOrd 어쩌구... 하는 에러를 볼 수 있는데,

표준 라이브러리 트레잇에 존재하는 PartialOrd 트레잇을 지정해서,

largest 함수가 비교 가능한 어떤 타입의 슬라이스에 대해 작동하도록 할 필요가 있다. 

```rust 
fn largest<T: PartialOrd>(list: &[T]) -> T {...}
```

위와 같이 바꾸면 `cannot move out of type [T], a non-copy array` 에러를 만나는데, 

이는 largest 함수를 제네릭으로 바꿨을 때, 

list[0]의 값을 largest 변수로 온전히 소유권을 넘길 수 있는가에 대한 문제가 발생한다.

i32나 char와 같은 타입들은 표준 라이브러리 트레잇인 `Copy` 트레잇으로 구현되어 있다. 

따라서 이를 추가함으로써 올바른 완전체 largest 함수를 만들 수 있다.

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    ...
}
```