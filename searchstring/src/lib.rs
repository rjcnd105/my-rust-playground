use std::error::Error;
use std::fs::File;

//  std::io::prelude 는 파일 I/O를 포함한 I/O 작업을 위해 유용한 다양한 특성이 있음
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // &'static str는 Err로 뱉는 "not enough arguments"를 의미.
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// 특성 오브젝트 Box - Box<Error>는 함수가 Error 특성을 구현하는 타입을 반환
// 이런 방식은 다양한 에러 상황에 다른 타입의 오류 값을 반환 할 수 있는 유연성을 확보할 수 있다.
pub fn run(config: Config) -> Result<(), Box<Error>>{
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    // Ok (())구문은 조금 이상하게 보일 수 있지만
    // ()를 사용하는 것과 마찬가지로 이는 사이드이펙트 없이 run을 호출하는 것을 나타내는 관용적인 방법입니다. 우리가 필요로 하는 값을 반환하지 않습니다.
    Ok(())
}