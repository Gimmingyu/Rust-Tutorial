fn main() {
    let v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("{}", i);
    }

    let mut vec = vec![1, 2, 3, 4, 5];

    for i in &mut vec {
        *i += 50;
        println!("{}", *i);
    }

    let vec = vec![5, 6, 7, 8, 9];
    for i in &vec {
        println!("{}", i);
    }

    let mut v = vec![11, 12, 13, 14, 15, 16, 17, 18, 19];
    for i in &mut v {
        *i += 2;
        println!("{}", i);
    }

    v.push(2147483645);
    for i in &mut v {
        *i += 2;
        println!("{}", i);
    }
}