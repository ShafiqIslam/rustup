fn main() {
    constants();
    scopes_and_shadowing();
    numeric_data_types_and_operations();
    characters();
    tuples();
    arrays();
    statement_and_expression();
    functions();
    conditionals();
    loops();

    exercises();
}

fn constants() {
    println!("\n\n--- Constants ---\n");
    const PI: f32 = 3.141592;
    println!("The value of PI is: {PI}");
}

fn scopes_and_shadowing() {
    println!("\n\n--- Scopes and Shadowing ---\n");
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn numeric_data_types_and_operations() {
    println!("\n\n--- Numeric Data types And Operations ---\n");

    println!("Min i8: {}", i8::MIN);
    println!("Min i32: {}", i32::MIN);
    println!("Min i64: {}", i64::MIN);
    println!("Min i128: {}", i128::MIN);
    println!("Min isize: {}", isize::MIN);
    println!("Max u8: {}", u8::MAX);
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);

    let double = 1.111111111111111;
    let single: f32 = 1.111111111111111;

    println!("single precision: {}", single + 0.111111111111111);
    println!("double precision: {}", double + 0.111111111111111);

    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    println!("56.7 / 32.2 = {quotient}, -5 / 3: {truncated}");
    let _remainder = 43 % 5;
}

fn characters() {
    println!("\n\n--- Characters ---\n");
    let _c = 'z';
    let unicode: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("unicode: {unicode}, heart_eyed_cat: {heart_eyed_cat}");
}

fn tuples() {
    println!("\n\n--- Tuples ---\n");
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of 2nd position of tuple is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;
}

fn arrays() {
    println!("\n\n--- Arrays ---\n");
    let _a = [1, 2, 3, 4, 5];

    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a = [3; 5];

    let first = a[0];
    let _second = a[1];

    println!("The value of 1st position of array is: {first}");
}

fn statement_and_expression() {
    println!("\n\n--- Expression and statement ---\n");
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn functions() {
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

fn conditionals() {
    println!("\n\n--- Conditionals ---\n");

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn loops() {
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

fn exercises() {
    println!("\n\n--- Exercises ---\n");
    fahrenheit_celsius_converter();
    fibonacci();
    christmas_carol();
}

fn fahrenheit_celsius_converter() {
    println!("\n\n--- Fahrenheit and Celsius Converter  ---\n");

    let temp: i32 = 98;
    let input_type = "C"; // C or F

    match input_type {
        "C" => println!("{}°C = {}°F", temp, celsius_to_fahrenheit(temp)),
        "F" => println!("{}°F = {}°C", temp, fahrenheit_to_celsius(temp)),
        _ => println!("t = {:?}", input_type),
    }
}

fn celsius_to_fahrenheit(c: i32) -> i32 {
    (c * (9 / 5)) + 32
}

fn fahrenheit_to_celsius(f: i32) -> i32 {
    (f - 32) * (5 / 9)
}

fn fibonacci() {
    println!("\n\n--- Fibonacci Series ---\n");

    for i in 0..15 {
        println!("fibonacci ({}) => {}", i, fib(i));
    }
}

fn fib(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn christmas_carol() {
    println!("\n\n--- The Twelve Days of Christmas ---\n");

    for day in 1..13 {
        day_intro(day);

        for gift_day in (1..(day + 1)).rev() {
            gift(
                gift_day,
                if gift_day == 1 && day != 1 {
                    "and "
                } else {
                    ""
                },
            );
        }
    }
}

fn day_intro(n: u32) {
    let day = match n {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "",
    };

    println!(
        "\nOn the {} day of Christmas\nmy true love sent to me:",
        day
    );
}

fn gift(n: u32, prefix: &str) {
    let gift_text = match n {
        1 => "a Partridge in a Pear Tree",
        2 => "Two Turtle Doves",
        3 => "Three French Hens",
        4 => "Four Calling Birds",
        5 => "Five Golden Rings",
        6 => "Six Geese a Laying",
        7 => "Seven Swans a Swimming",
        8 => "Eight Maids a Milking",
        9 => "Nine Ladies Dancing",
        10 => "Ten Lords a Leaping",
        11 => "Eleven Pipers Piping",
        12 => "12 Drummers Drumming",
        _ => "",
    };

    println!("{}{}", prefix, gift_text);
}
