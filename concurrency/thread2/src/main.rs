use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap(); // #2. 고로 여기서 하면 1-9까지 출력 후 1-5까지 출력

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // #1. 핸들에 대한 스레드가 종료될 때 까지 현재 실행중인 스레드 블록.
}