mod christmas_carol;
mod employee_db;
mod fahrenheit_celsius_converter;
mod fibonacci;
mod median_and_mod;
mod pig_latin;

pub fn common_programming_exercises() {
    println!("\n\n--- Exercises ---\n");
    fahrenheit_celsius_converter::fahrenheit_celsius_converter();
    fibonacci::fibonacci();
    christmas_carol::christmas_carol();
}

pub fn collection_exercises() {
    println!("\n\n--- Collection exercises ---\n");
    median_and_mod::median_and_mod();
    pig_latin::pig_latin();
    employee_db::employee_db();
}
