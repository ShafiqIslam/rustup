pub fn functions() {
    println!("\n\n--- Functions ---\n");

    let x = five();
    println!("The value of five() is: {x}");

    let x = plus_one(5);
    println!("The value of plus_one(5) is: {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
