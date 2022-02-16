use std::env;

mod types;
mod control_flow;
mod ownership;
mod structs;
mod enums;
mod server;

fn main() {
    let args: Vec<String> = env::args().collect();
    let build_type = &args[1];

    if build_type == "server" {
        server::print();
    } else {
        // ferris_says::print();
        // rand::guess();
        types::print();
    
        control_flow::print();
    
        ownership::print();
    
        structs::print();
    
        enums::print();
    }

    println!("Hello, world!");
}
