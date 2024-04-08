pub fn strings_are_complicated() {
    println!("\n\n--- Strings are complicated ---\n");

    initiate_strings();
    utf8_strings();
    update_strings();
    index_of_string();
}

fn initiate_strings() {
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    println!("{s}");
}

fn utf8_strings() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Hola");
    let hello = String::from("Здравствуйте");
    println!("{hello}");
}

fn update_strings() {
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
}

fn index_of_string() {
    let s1 = String::from("hello");
    // let h = s1[0]; // strings can not be indexed

    let hello = "Здравствуйте";
    // let f = &hello[0]; // because utf is 2 butes long, and can produce wrong result

    // also there are issues with grapheme clusters.
    let hello = "नमस्ते";
    for c in hello.chars() {
        print!("{c}, ");
    }
    println!("");

    for b in hello.bytes() {
        print!("{b}, ");
    }
    println!("");
}
