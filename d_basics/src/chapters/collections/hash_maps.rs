use std::collections::HashMap;

pub fn hash_maps() {
    println!("\n\n--- HashMaps ---\n");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    scores.insert(String::from("Blue"), 25);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.entry(String::from("Green")).or_insert(50);
    println!("{:?}", scores);

    frequency();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name, field_value has been moved to the map's heap
}

fn frequency() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
