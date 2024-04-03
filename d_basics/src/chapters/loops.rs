pub fn loops() {
    println!("\n\n--- Loops ---\n");

    basic_loop();
    labeled_loop();
    while_loop();
    for_loop();
}

fn basic_loop() {
    println!("\n\n--- Basic Loop ---\n");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn labeled_loop() {
    println!("\n\n--- Labeled Loop ---\n");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    println!("\n\n--- While Loop ---\n");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    println!("\n\n--- For Loop ---\n");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        print!("{element}, ");
    }

    println!("");
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
