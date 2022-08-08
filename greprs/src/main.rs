use std::env;
// std::env::args 함수는 반복자 형식으로 커맨드라인 인자들을 전달해준다. 
use std::fs::File;
use std::io::prelude::*;


fn main() {
    // collect 함수를 호출해서 반복자가 생성하는 일련의 값들을 벡터로 변환..
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    
    let query = &args[1];
    let filename = &args[2];

    println!("{:?}", query);
    println!("{:?}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong reading the file");
    let first_line = get_next_line(contents.as_str());

    // println!("With text:\n{}", contents);
    println!("{}", first_line);
}

fn get_next_line(contents: &str) -> String {
    let mut single_sentence = String::new();
    for i in contents.chars() {
        if i != '\n' {
            single_sentence = format!("{}{}", single_sentence, i);
        } else {
            break;
        }
    }
    single_sentence
}