extern crate searchstring;

use std::env;
use std::process;

// std::env::args 함수는 반복자(iterator) 형식으로 커맨드라인 인자들을 우리 프로그램에 전달해줍니다.
// 반복자에 collect 함수 호출을 통해 반복자가 생성하는 일련의 값을 벡터로 변환할 수 있습니다.
use std::io::prelude::*;

use searchstring::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // unwrap_or_else를 사용하면 panic!이 아닌 에러 처리를 직접 정의 할 수 있음.
    // Ok로 포장한 내부 값을 반환함. 그러나 Err값이면 메소드는 closure의 코드를 호출합니다.
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);

        // process::exit함수는 프로그램을 즉시 중단시키고 종료 상태 코드로 전달받은 값을 반환합니다.
        // 이것은 panic!기반의 처리 방식과 유사해 보이지만, 더이상 필요하지 않은 출력을 하지 않죠.
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // if let을 사용하여 run이 Err값을 반환하는지 검사하고 만약 그렇다면 process::exit(1)을 호출
    // unwrap_or_else을 쓸 필요가 없다. Ok시 값은 무조건 ()일테니까
    if let Err(e) = searchstring::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
