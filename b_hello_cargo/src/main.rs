use std::io;

fn main() {
    println!("What's your name?");
    let mut name = String::new();
    let greet = "Nice to meet you";

    io::stdin().read_line(&mut name).expect("Didn't receive input.");

    println!("Hello, {}! {}.", name.trim_end(), greet);
}
