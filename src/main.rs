// mod demo;
// use demo::ferris_says;
// use demo::rand;

mod types;

use types::number;
use types::string;

fn test_types() {
    println!();
    println!();
    println!("================== types start ================");
    number::print();
    string::print();
    println!("================== types end ================");
    println!();
    println!();
}

fn main() {
    // ferris_says::print();
    // rand::guess();

    test_types();

    println!("Hello, world!");
}

