use std::thread;
use std::time::Duration;

// 클로저(closure)
// 클로저는 노출된 인터페이스에서 사용되지 않는다.
// 좁은 문맥 안에서 사용되며 그 문맥 안에서는 컴파일러가 파라미터와 리턴타입을 추론할 수 있다. (그러므로 보통 쓰지 않는다는 말인듯)
// let expensive_closure = |num: u32| -> u32 { 처럼 타입 정의도 가능

// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x|             { x + 1 };
// let add_one_v4 = |x|               x + 1  ;

// Fn 트레잇은 표준 라이브러리에서 제공 합니다.
// 모든 클로저들은 다음 트레잇 중 하나를 구현 합니다: Fn, FnMut, 혹은 FnOnce.

// 클로저는 함수에 없는 추가적인 능력을 갖고 있음: 환경을 캡처해서 클로저가 정의된 스코프의 변수들을 접근할 수 있다.

/*
// 아래의 예제에서 클로저는 main의 스코프에 접근 할 수 있음.
fn main() {
    let x = 4;

    // fn equal_to_x(z: i32) -> bool { z == x } // 함수로는 접근 불가능!
    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y)); // PASS
}
 */

/***
* 세가지 방식으로 그들의 환경에서 값을 캡처 할 수 있는데, 함수가 파라미터 를 받는 세가지 방식과 직접 연결 됩니다
소유권 받기, 불변으로 빌려오기, 가변으로 빌려오기. 이것들은 다음과 같이 세개의 Fn 트레잇으로 표현 합니다:
*
* Fn(불변으로 빌려오기)     :  그 환경으로 부터 값들을 불변으로 빌려 옵니다.
* FnMut(가변으로 빌려오기)  :  값들을 가변으로 빌려오기 때문에 그 환경을 변경할 수 있습니다.
* FnOnce(소유권 받기)     :  그 클로저의 환경으로 알고 있는, 그것을 둘러싼 환경에서 캡처한 변수 들을 소비합니다.
                          캡처한 변수를 소비하기 위해, 클로저는 이 변수의 소유권을 가져야 하고 그것이 정의될 때 클로저 안으로 그것들을 옮겨와야 합니다.
                          이름의 일부인 Once 는 그 클로저가 동일한 변수들에 대해 한번이상 소유권을 얻을수 없다는 사실을 의미하며, 그래서 한 번만 호출 될 수 있습니다.
* 기본적으론 Fn 트레잇.
***/

fn el_move() {
    let x = vec![1, 2, 3];

    // move 키워드는 소유권을 가져온다.
    // 쓰레드에서 주요 사용됨. 쓰레드가 사용하는 동안 캡쳐 당시의 불변의 값을 유지하기 위함.
    /**
        let x = 5;

        std::thread::spawn(move || {
            println!("captured {} by value", x)
        }).join().unwrap();

        // x is no longer available
    **/
    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

// 지연평가를 위한 클로저 메모이제이션
impl<T> Cacher<T>
    where T: Fn(u32) -> u32 // 32-bit unsigned integer type
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 { // life cycle은 self
        match self.value {
            Some(v) => v, // 있으면 그대로 반환
            // 없으면 계산
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result(intensity)
            );
        }
    }
}

fn main() {

    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}


#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}