fn main() {
    let string1 = String::from("abcd");
    {
        let string2: &str = "xyzaaa";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
        println!("string2: {}", string2);
    }

    println!("string1: {}", string1);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}