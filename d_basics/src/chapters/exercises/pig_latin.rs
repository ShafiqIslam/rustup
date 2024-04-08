pub fn pig_latin() {
    println!("Pig latin");
    println!("first => {}", convert_to_pig_latin("first".to_string()));
    println!("apple => {}", convert_to_pig_latin("apple".to_string()));
}

fn convert_to_pig_latin(s: String) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let first_char: char = s.chars().next().unwrap();

    if vowels.contains(&first_char) {
        return s + "-hay";
    }

    return format!("{}-{first_char}ay", s.split_at(1).1);
}
