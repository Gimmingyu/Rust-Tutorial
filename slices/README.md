# Slices

레퍼런스를 빼고, 소유권을 갖지 않는 또 다른 데이터 타입은 슬라이스가 있다.

슬라이스는 컬렉션 전체가 아닌 연속된 일련의 요소들을 참조할 수 있게 한다.

예제로서, 스트링을 입력받아 첫 번째 단어를 찾아주는 함수가 있다고 가정했을 때,

이 함수의 시그니처에 대해 생각해보자.

```rust
fn first_word(s: &String) -> ? 
```

이 함수 first_word는 &String을 인자로 받는다. 

소유권을 넘기지 않는게 목적이니 이렇게 하는 것이 좋다.

그런데 무엇을 반환해야 할 지가 애매하다. 

스트링의 일부에 대해 표현을 어떻게 하는 것이 좋을까? 

```rust

#![allow(unused)]
// size를 반환한다.
fn first_word(s: &String) -> usize {
    // as_bytes 메소드를 사용하면 바이트 배열로 변환되고, 각 값이 공백인지 확인이 가능하다.
    let bytes = s.as_bytes();
    // enumerate 를 이용해서 인덱스와 값을 받아온다.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

위와 같이 단어 끝부분의 인덱스를 반환할 수는 있다. 

그러나 이는 좋은 해결 방법이라고는 볼 수 없다.

## 스트링 슬라이스

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let same_hello = &s[..5];
let world = &s[6..11];
let same_world = &s[..11];
```

슬라이스는 전체 String의 참조자를 갖는 것과 비슷하지만, 일부분에 대한 참조자이다.

물론 힙에 존재하는 데이터를 복사하지는 않고, 단순히 가리키는 위치만을 참조하게 된다.