use std::cmp::PartialOrd;

// T타입 제네릭을 사용하는 구조체 Point<T>
struct Point<T> {
    x: T,
    y: T,
}

//  T타입의 x필드에 대한 참조자를 반환하는 Point<T> 구조체 상에 x라는 메소드 정의
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// T, U타입을 사용하는 구조체
struct complexPoint<T, U> {
    x: T,
    y: U,
}

// 구조체 정의와는 다른 제네릭 타입을 사용하는 메소드
impl<T, U> complexPoint<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> complexPoint<T, W> {
        complexPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn generic_largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let result = generic_largest(&numbers);

    println!("largest = {:?}", result);
}
