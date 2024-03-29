use std::env;

mod types;
mod control_flow;
mod ownership;
mod structs;
mod enums;
mod server;
mod manage_project;
mod collections;
mod error;
mod generic_types;

fn main() {
    let args: Vec<String> = env::args().collect();
    let build_type = args.get(1);
    if build_type == Some(&"server".to_string()) {
        server::start();
        return;
    }

    // ferris_says::print();
    // rand::guess();
    types::print();

    control_flow::print();

    ownership::print();

    structs::print();

    enums::print();

    manage_project::print();

    collections::print();

    error::print();

    generic_types::print();

    println!("Hello, world!");
}
