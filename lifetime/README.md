# Lifetime

러스트에서 모든 참조자는 라이프타임을 갖는데, 이는 해당 참조자가 유효한 스코프를 말한다.

대부분의 경우 라이프타임 또한 타입을 추론하는 것과 같이 암묵적으로 추론된다.

러스트는 제네릭 라이트타임 파라미터를 이용하며 우리에게 명시하기를 요구한다.

---

## 함수에서의 제네릭 라이프타임

```rust
{
    let r;

    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);
}
```

위의 예시는 스코프 밖으로 벗어난 값에 대한 참조자를 사용하는 시도이다.

내부 스코프에서 x의 값을 5로 초기화하고, x의 참조자를 r에 대입했지만, 내부 스코프는 끝난다.

그러나 스코프가 끝나면서 x는 Drop되었을 것이고, 러스트 컴파일러는 댕글링 참조자를 허용하지 않는다.

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

위와 같은 상황에서, 이 파일은 컴파일 되지 않는다.

```Shell
error[E0106]: missing lifetime specifier
  --> src/main.rs:12:33
   |
12 | fn longest(x: &str, y: &str) -> &str {
   |               ----     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
   |
12 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
   |           ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
```

반환 타입에 대한 제네릭 라이프타임 파라미터가 필요하다는 에러를 얻을 수 있는데,

반환되는 참조자가 x가 될지, y가 될지는 아직 모르기 때문이다.

그 말은 곧 우리가 반환하는 참조자가 언제까지 유효할 지 추론하기가 어렵다는 뜻이다.

그래서 우리는 참조자들 간의 관계를 정의하는 제네릭 라이프타임 파라미터를 추가하여 빌림 검사기가 분석할 수 있도록 돕는다.

---

## 라이프타임 명시 문법

라이프타임 명시가 실제로 라이프타임을 바꿀 수는 없다.

함수의 시그니처가 제네릭 타입 파라미터를 특정할 때 이 함수가 어떤 타입이든 허용할 수 있는 것과 같은 방식으로,

함수의 시그니처가 제네릭 라이프타임 파라미터를 특정할 때라면 이 함수는 어떠한 라이프타임을 가진 참조자라도 허용할 수 있다.

라이프타임 명시의 문법은 `어퍼스트로피`(`'`)로 시작한다.

```rust
&i32        : a reference
&'a i32     : a reference with an explicit lifetime
&'a mut i32 : a mutable reference with an explicit lifetime
```

아까의 예시를 컴파일 오류 없이 실행하려면 다음과 같이 실행할 수 있다.

```rust

// 시그니처 내의 모든 참조자들이 동일한 라이프타임 'a를 가지고 있어야 함을 특정한 longest 함수...
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

이 예시를 통해 우리는 빌림 검사기의 능력을 엿볼 수 있다.

```rust
fn main() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = "xyz";
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

위의 예시는 컴파일 에러가 발생한다. 왜냐하면 string2가 내부스코프에서 선언 및 초기화되고,

내부스코프를 벗어나면 drop되기 때문에 string2가 외부 스코프의 끝까지 유효할 필요가 있다.

그렇지 않다면 러스트는 x와 y의 라이프타임이 동일하지 않음을 알고 오류를 미연에 방지하기 위해 컴파일 에러를 발생시킨다.

---

## 구조체 정의 상에서의 라이프타임 명시

구조체가 참조자를 들고 있을 수 있지만, 그 경우 구조체 정의 내의 모든 참조자들에 대해 라이프타임을 명시해야한다.

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find '.'");
    let i = ImportantExcerpt { part: first_sentence };
}
```

## 라이프타임 생략

러스트의 예시로 작성했던 몇 함수들을 보면 라이프타임 명시 없이 컴파일 되는 경우가 많다.

이는 코드를 중복 작성하는 것을 피하고, 빌림 검사기가 라이프타임을 추론할 수 있도록 하는 것인데,

라이프타임 생략 규칙이 있다. 이 규칙들은 컴파일러가 고려하는 특정 경우의 집합이고, 이러한 경우에는 라이프타임 생략이 가능하다.

1. 참조자인 각각의 파라미터는 고유한 라이프타임 파라미터를 갖는다. 

하나의 파라미터를 갖는 함수는 하나의 라이프타임 파라미터

```rust
fn foo<'a>(x: &'a i32)
```

두 개의 파라미터를 갖는 함수는 두 개의 라이프타임 파라미터를 가지는 경우...

```rust
fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
```

2. 만일 정확히 딱 하나의 라이프타임 파라미터만 있다면, 그 라이프타임이 모든 출력 라이프타임 파라미터들에 대입된다. 

```rust
fn foo<'a> (x: &'a i32) -> &'a i32
```

3. 만일 여러 개의 입력 라이프타임 파라미터가 있는데, 메소드라서 그중 하나가 &self 혹은 &mut self라고 한다면, 
    self의 라이프타임이 모든 출력 라이프타임 파라미터에 대입된다.

---

