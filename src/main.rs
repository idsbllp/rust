mod types;
mod control_flow;
mod owner_ship;
mod structs;
mod enums;

fn main() {
    // ferris_says::print();
    // rand::guess();
    types::print();

    control_flow::print();

    structs::print();
    
    enums::print();

    owner_ship::print();

    println!("Hello, world!");
}
