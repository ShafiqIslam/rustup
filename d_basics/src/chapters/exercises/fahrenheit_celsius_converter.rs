pub fn fahrenheit_celsius_converter() {
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
