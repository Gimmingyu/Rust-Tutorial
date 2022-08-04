// cargo new "$(NAME)" --lib 으로 라이브러리 크레이트 생성가능.
// mod = Module을 의미한다.
// 러스트 내의 모듈 정의는 모두 mod로 시작.
// 다른 모듈에서 connect 함수를 사용한다면, 네임스페이스 문법을 사용해야한다
// ex) network::connect()
pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}