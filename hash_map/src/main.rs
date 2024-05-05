use std::collections::HashMap;

// HashMap도 String, Vec와 마찬가지로 데이터를 힙에 저장한다.
// HashMap은 String타입의 키와 i32타입의 값을 가진다.
// 벡터와 비슷하게 모든 키는 같은 타입이어야 하고, 모든 값은 같은 타입이어야 한다.

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:#?}", scores);

    let teams = vec![String::from("Red"), String::from("Green")];
    let initial_score = vec![10, 50];

    let scores: HashMap<&String, &i32> = teams.iter().zip(initial_score.iter()).collect();
    // collect()를 사용하면 다른 많은 데이터 구조로 바뀔 수가 있는데,
    // <_, _> 를 사용함으로써 벡터에 담긴 값으로 키와 값의 타입을 유추해서 해시 맵을 생성한다.

    println!("{:#?}", scores);

    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 50);

    scores.entry("Blue".to_string()).or_insert(100);

    println!("{:#?}", scores);

    let text = "hello world wonderful world".to_string();
    let mut map = HashMap::new();
    for i in text.split_whitespace() {
        let count = map.entry(i).or_insert(0);
        *count += 1;
        println!("{}", i);
    }

    println!("{:#?}", map);
}
