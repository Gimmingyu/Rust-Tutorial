# Generic

모든 프로그래밍 언어는 컨셉의 복제를 다루기 위한 도구를 가지고 있다. 

제네릭은 구체화된 타입이나 다른 속성들에 대하여 추상화된 대리인이다. 

우리는 제네릭이 실제로 어떻게 완성되는지 알 필요 없이, 

제네릭의 동작 혹은 다른 제네릭과 어떻게 연관되는지와 같은 속성을 표현할 수 있다.

### 제네릴 타입을 이용하지 않는 중복 코드 다루기

```rust

let numbers = vec![34, 50, 25, 100, 65];

let mut largest = numbers[0];

for number in numbers {
    if number > largest {
        largest = number;
    }
}
```