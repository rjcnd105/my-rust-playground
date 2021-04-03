extern crate my_test;

/*
코드의 상단에 extern crate mytests 추가했는데, 이는 단위 테스트에서는 필요 없었지요.
이는 tests 디렉토리 내의 각 테스트가 모두 개별적인 크레이트이라서, 우리의 라이브러리를 각각에 가져올 필요가 있기 때문입니다.

tests/integration_test.rs에는 #[cfg(test)]를 이용한 어노테이션을 해줄 필요가 없습니다.
카고는 test 디렉토리를 특별 취급하여 cargo test를 실행시켰을 때에만 이 디렉토리 내의 파일들을 컴파일합니다.
*/

/*
#[test] 는 꼭 붙혀주어야함.
*/

#[test]
fn it_adds_two() {
    assert_eq!(5, my_test::add_two(2));
}