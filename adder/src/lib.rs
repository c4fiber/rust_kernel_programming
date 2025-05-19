pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// cfg(test) attribute는 이어지는 코드가 테스트 환경에서만 컴파일되도록 합니다.
#[cfg(test)]
mod tests {
    use super::*; // 상위 스코프의 모든 항목을 현재 테스트 모듈로 가져옵니다.

    // it_works 함수는 테스트 함수 입니다.
    #[test]
    fn it_works() {
        let result = add(2, 2); // add 함수에 2와 2를 전달하고 결과를 받습니다.
        assert_eq!(result, 4); // 결과가 4와 같은지 확인합니다. 테스트 통과: 아무 출력 없음, 테스트 실패: 패닉
    }
}
