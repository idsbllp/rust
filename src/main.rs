mod types;
mod control_flow;
mod ownership;
mod structs;
mod enums;

fn main() {
    // ferris_says::print();
    // rand::guess();
    types::print();

    control_flow::print();

    structs::print();
    
    enums::print();

    ownership::print();

    println!("Hello, world!");
}
