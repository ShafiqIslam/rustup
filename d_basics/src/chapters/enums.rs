#![allow(dead_code)]
#![allow(unused_variables)]

pub fn enums() {
    println!("\n\n--- Enums and Pattern Matching ---\n");

    basic_enums();
    enums_in_struct();
    enum_values();
    enum_with_method();
    match_flow();
}

fn basic_enums() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("basic enums: {:?}, {:?}", four, six);
}

fn enums_in_struct() {
    let home = IpAddr1 {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr1 {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("enums with value with struct: {:?}, {:?}", home, loopback);
}

fn enum_values() {
    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    println!("value enums without struct: {:?}, {:?}", home, loopback);

    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));
    println!("value enums without struct: {:?}, {:?}", home, loopback);
}

fn enum_with_method() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn match_flow() {
    value_in_cents(Coin::Quarter);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("option enums: {:?}, {:?}, {:?}", five, six, none);

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("Control flow with if-let: {}", max);
    }
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr1 {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    Write(String),
    _ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("basic enums: {:?}", self);
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    Other,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => {
            println!("This is a quarter.");
            25
        }
        _ => {
            println!("Match are exhautive.");
            0
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
