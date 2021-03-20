fn greet_world() {
    println!("Hello, world!");

    let korean = "안녕하세요!";
    let japan = "こんにちは";

    let regions = [korean, japan];

    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main(){
    greet_world();
}