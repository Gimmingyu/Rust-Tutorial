fn main() {
    let fibo = fibonazzi(5);

    println!("fibo: {}", fibo);
}


fn fibonazzi(x: i32) -> i32 {
    if x == 1 {
        return 0
    } else if x == 2 {
        return 1
    } else if x == 3 {
        return 1
    }
    fibonazzi(x - 2) + fibonazzi(x - 1)
}
