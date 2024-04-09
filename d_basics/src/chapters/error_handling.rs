#![allow(unused_variables)]

use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

pub fn error_handling() {
    println!("\n\n--- Error Handling ---\n");

    println!("Don't panic!");
    if false {
        panics();
        recover_with_result();
    }
}

fn panics() {
    panic!("Crash");
}

fn recover_with_result() {
    basic_result_usage();
    catch_specific_error_type();
    panic_shortcut_with_expect();

    propagate_error();
}

fn basic_result_usage() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn catch_specific_error_type() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn panic_shortcut_with_expect() {
    let greeting_file = File::open("hello.txt").expect("Problem opening the file");
}

fn propagate_error() {
    let _ = read_username_from_file_full();
    let _ = read_username_from_file_shortcut();
    let _ = fs::read_to_string("hello.txt");
    question_operator_can_be_used_with_option_also();
}

fn read_username_from_file_full() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn question_operator_can_be_used_with_option_also() -> Option<char> {
    "".lines().next()?.chars().last()
}
