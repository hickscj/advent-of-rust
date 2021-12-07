// use std::env;
use std::fs;

fn main() {
    let contents: String = fs::read_to_string("input.txt")
        .expect("Something went wrong");

    let mut x: i32 = 0;
    for num in contents.chars() {
        println!("{}", num);
        x = x + 1;
    }
}

fn test_function() {
    println!("This is the test function");
}