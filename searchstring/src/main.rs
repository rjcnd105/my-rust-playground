extern crate searchstring;

use std::env;
use std::process;

// std::env::args 함수는 반복자(iterator) 형식으로 커맨드라인 인자들을 우리 프로그램에 전달해줍니다.
// 반복자에 collect 함수 호출을 통해 반복자가 생성하는 일련의 값을 벡터로 변환할 수 있습니다.
use std::io::prelude::*;

use searchstring::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = searchstring::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}