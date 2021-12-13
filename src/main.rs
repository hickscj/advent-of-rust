// use std::env;
// use std::fs;

fn main() {
    print_examples();
}

fn print_examples() {
    /*
     * https://doc.rust-lang.org/rust-by-example/hello/print.html 
     */
    println!("---------------------");
    println!("Examples of printing statement formats in Rust");
    println!("---------------------");
    println!("{} days", 31);

    // variable interpolation
    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");
    
    // special formatting
    println!("{} of {:b} people know binary", 1, 2);

    // right-align text
    println!("{number:>width$}", number=1, width=16);

    // pad numbers with extra zeroes
    println!("{number:0>width$}", number=1, width=6);

    // create a structure named `Structure` which contains an i32
    #[derive(Debug)]
    struct Structure {
        id: i32
    }

    let a_struct = Structure { id: 12 };

    // custom types require more complex handling
    // println!("This struct `{}` won't print...", Structure(3));
     println!("This struct {:?} WILL", a_struct.id);
}