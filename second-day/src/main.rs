use std::fs;

fn main() {
    let contents = fs::read_to_string("../res/first-day/input.txt").expect("The path is wrong!");
    println!("Hello, world!");
}
