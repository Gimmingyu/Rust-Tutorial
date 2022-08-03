fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // Of course You can also use the following statements...
    println!("condition was {}", number > 5);

    let x = 3;

    // if x {
    //     println!("x was three");
    // }
    // Error ! expected `bool`, found integer

    if x != 0 {
        println!("x was {}", x);
    }

    
    // let condition = true;
    // let number = if condition {
    //     5
    // } else {
    //     "six"
    //     // Error ! expected integral variable, found reference
    // };
    // if block이 정수형을 산출하는 식이고 else block은 문자열을 산출하는 식이다. 
    // 변수가 가질 수 있는 타입은 오직 하나이기 때문에 Error 발생.
    // println!("The value of number is: {}", number);
}
