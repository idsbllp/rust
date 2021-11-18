use ferris_says::say;
use rand::Rng;
use std::io::{stdin, stdout, BufWriter};
use std::cmp::Ordering;

#[allow(dead_code)]
fn guess() {
    let secret_number: i32 = rand::thread_rng().gen_range(1..101);

    println!("secret_number: {}", secret_number);

    loop {
        println!("Please input an number: ");
        let mut guess = String::new();
    
        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: i32 = match guess.trim().parse() {
            Ok(sum) => sum,
            Err(_) => continue,
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less"),
            Ordering::Equal => {
                println!("Equal!");
                break;
            },
            Ordering::Greater => println!("Greater"),
        }

        println!("Your number: {}", guess);
    }
}

fn main() {
    // cow_say();

    println!("Hello, world!");
    println!("Hello, world!");


    guess();
}

#[allow(dead_code)]
fn cow_say() {
    let stdout = stdout();
    let message = String::from("Hello Rust, are you ok?");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
