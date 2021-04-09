use std::error::Error;
use std::fs::File;
use std::env;

//  std::io::prelude 는 파일 I/O를 포함한 I/O 작업을 위해 유용한 다양한 특성이 있음
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // &'static str는 Err로 뱉는 "not enough arguments"를 의미.
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

// 특성 오브젝트 Box - Box<Error>는 함수가 Error 특성을 구현하는 타입을 반환
// 이런 방식은 다양한 에러 상황에 다른 타입의 오류 값을 반환 할 수 있는 유연성을 확보할 수 있다.
// trait의 경우에는 앞에 dyn을 붙혀줘야함. 그리고 사이즈가 정해지지 않았기 때문에 Box로 감싸야됨.
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{

    // ?는 Result를 반환하는 함수에서만 사용될 수 있습니다
    // https://rinthel.github.io/rust-lang-book-ko/ch09-02-recoverable-errors-with-result.html#에러를-전파하기-위한-숏컷-
    // ?는 Ok내의 값을 변수 f에게 반환해줄 것입니다. 만일 에러가 발생하면 ?는 전체 함수로부터 일찍 빠져나와 호출하는 코드에게 어떤 Err 값을 줄 것입니다.
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;


    for line in search(&config.query, &contents) {
        println!("{}", line)
    }

    // Ok (())구문은 조금 이상하게 보일 수 있지만
    // ()를 사용하는 것과 마찬가지로 이는 사이드이펙트 없이 run을 호출하는 것을 나타내는 관용적인 방법입니다. 우리가 필요로 하는 값을 반환하지 않습니다.
    Ok(())
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}