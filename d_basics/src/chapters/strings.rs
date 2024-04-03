pub fn strings() {
    println!("\n\n--- Strings ---\n");

    let mut st1: String = String::new();
    st1.push('A');
    st1.push_str(" word");

    for word in st1.split_whitespace() {
        print!("{}, ", word);
    }

    let st2: String = st1.replace("A", "Another");
    println!("\n{}", st2);

    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for c in v1 {
        if c == ' ' {
            continue;
        }
        print!("{c}, ");
    }

    let st4: &str = "Random String";
    let mut st5: String = st4.to_string();
    println!("\n{st5}");

    let _byte_array = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("Length: {}", st6.len());
    st5.clear();

    let st6 = String::from("Just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7;
    for c in st8.bytes() {
        print!("{c}");
    }
    println!("");
}
