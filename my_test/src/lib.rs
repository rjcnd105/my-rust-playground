// cargo test

/* assert!, panic! */
// assert! 로 참/거짓 테스트
// 정확히 같은 값 인지 테스트할때 assert_eq!
// 같은 값이 되지 말아야할때 assert_ne! (assert_eq가 아닐때 성공)
// panic! 으로 테스트 실패 처리
// 다음 인자로 메세지 작성도 가능.

/* #[derive(...)] */
// 단언에 실패한 경우 추론 가능할때 값을 출력해주는 어노테이션.
// 추론 가능한 트레잇을 적어주면 됨.
// 주로 #[derive(PartialEq, Debug)] 를 많이 씀.


/* #[should_panic] */
// 코드가 panic을 일으키면 통과
// panic 을 일으키지 않으면 실패
// #[should_panic(expected = "...")]
// 처럼 특정 메세지의 패닉이 났을때만 통과로 할 수 있음.

/* #[ignore]를 추가해서 test 실행시 무시할 할 수 있음.*/
/* $ cargo test -- --ignored */
// 를 통해서 ignore 된 애들만 실행 가능


#[cfg(test)]
pub mod my_test {
    use super::*;

    // Pass
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // Fail
    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    // Pass
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 8 };

        assert!(larger.can_hold(&smaller));
    }

    // Pass
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 8 };

        assert!(!larger.can_hold(&smaller));
    }

    // Fail
    #[test]
    fn negative() {
        assert_ne!(5, add_two(2) )
    }

    // Fail
    #[test]
    fn name_test() {
        let result = greeting("cargo");
        assert!(result.contains("cargo"),
                "Greeting did not contain name(\"{}\"), value was \"{}\"", "cargo", result)
    }

    // Pass
    // #[should_panic] 땜에 오히려 panic 나서 통과함.
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Fail
    // 0이 아니면 실패함
    // (expected = MSG) MSG에 해당하는 panic 메세지가 아니면 에러남.
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100, got 0")]
    fn only_less_than_1() {
        Guess::new(3);
    }
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 5 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be greater than or equal to 5, got {}.", value);
        }

        Guess {
            value
        }
    }
}



#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}