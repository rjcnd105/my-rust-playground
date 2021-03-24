mod myFile;
mod traitTest;

use std::fs::File;
use myFile::read;

enum IpAddr {
    V4(String),
    V6(String),
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn main() {
    let arr = [1,2,3,4];
    let vec = vec![1,2,3,4];
    let home = IpAddr::V4(String::from("127.0.0.1"));

    // trait
    let tweet = traitTest::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };


    let f = File::open("Cargo.toml");

    println!("myFile: {:#?}",f);

    println!("Coin: {:?}", value_in_cents(Coin::Nickel));
    println!("arr {:?}, {:?}", arr, arr.iter());
    println!("vec {:?}, {:?}", vec, vec.iter());
    println!("{:?}", (0..10).zip(0..));
    for (x, y) in (0..).zip(10..) {
        if x + y > 100 {
            break;
        }
        // println!("x:{}, y:{}", x, y)
    }
}