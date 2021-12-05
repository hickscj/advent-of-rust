// use std::env;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong");

    let mut x = 0;
    for num in contents.chars() {
        println!("{}", num);
        x = x + 1;
    }
}
