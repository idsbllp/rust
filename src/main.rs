// mod demo;
// use demo::ferris_says;
// use demo::rand;

mod types;

fn test_types() {
    println!();
    println!();
    println!("================== types start ================");
    types::number::print();
    types::string::print();
    types::tuple::print();
    types::array::print();
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

