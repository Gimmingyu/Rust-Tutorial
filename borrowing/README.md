# Borrwing and Reference

러스트에도 물론 소유권을 넘기는 대신 개체에 대한 레퍼런스를 인자로 사용해 회피하는 방법이 있다.

## 불변참조자

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

calculate_length함수에 주소를 넘겼고, String이 아니라 &String을 인자로 받는다는 것에 주목하자.

&s1은 s1의 *값을 참조하지만 소유하지는 않는 참조자*를 생성하도록 해준다. 

소유권을 가지지 않기 때문에 스코프 밖으로 벗어나더라도 참조자가 가리키는 값은 버리지 않는다. 

물론 반환할 필요도 없다는 뜻이다. 함수의 파라미터로 참조자를 만드는 것을 러스트에서는 *빌림*이라고 부른다.

그러나 빌린 무언가를 고치려고 시도하면 에러가 발생한다.

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
    // Error ! 
}
```

## 가변 참조자

가변참조자를 이용하면 오류를 고칠 수 있다.

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

s를 mut으로 바꾸고, &mut s로 mutation, 가변 참조자를 생성하고 parameter도 &mut String으로 받아야 한다. 

가변참조자는 한 가지 큰 제한이 있는데, 특정 스코프 내에 특정한 데이터 조각에 대한 가변 참조자를 딱 하나만 만들 수 있다는 것이다. 

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
// Error ! cannot borrow `s` as mutable more than once at a time
```

이러한 제한이 귀찮은 점이 있지만 러스트가 컴파일 타임에 data race를 방지할 수 있도록 해준다.

데이터 레이스는 아래 세 가지 동작이 발생했을 때 나타나는 특정한 레이스 조건이다. 

1. 두 개 이상의 포인터가 동시에 같은 데이터에 접근한다.
2. 그 중 적어도 하나의 포인터가 데이터를 쓴다.
3. 데이터에 접근하는데 동기화를 하는 어떠한 메커니즘도 없다.

데이터 레이스는 undefined behavior를 일으키고 런타임에 이를 추적하고자 할 때는 진단하기가 어렵다. 

러스트는 참으로 친절하게도 이를 컴파일 단계에서 막아버린다. 

우리는 중괄호를 이용한 스코프를 이용해 이를 우회할 수 있다. 

```rust 
fn main() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // 여기서 r1은 스코프 밖으로 벗어났으므로, 우리는 아무 문제 없이 새로운 참조자를 만들 수 있습니다.

    let r2 = &mut s;
}
```

가변 참조자와 불변 참조자를 혼용할 경우 지켜야할 규칙이 있다. 

```rust

let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
let r3 = mut &s;

// Error ! cannot borrow `s` as mutable because it is also borrowed as immutable
```

불변 참조자를 가지고 있는 경우에도 *역시* 가변 참조자를 만들 수 없다. 

여러 개의 불변 참조자는 가능하지만, 가변 참조자는 특정 스코프 내에서 오로지 하나만 존재해야 한다. 

---

## 댕글링 참조자 

포인터가 있는 언어에서는 자칫하면 댕글링 포인터를 만들기가 쉽다. 

러스트에서는 참으로 친절하게도 컴파일러가 모든 포인터가 댕글링 포인터가 되지 않도록 보장해준다.
