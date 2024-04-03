pub fn slices() {
    problem_without_slice();
    solution_with_slice();
    array_slice();
}

fn problem_without_slice() {
    let mut s = String::from("hello world");
    let word = first_word_without_slices(&s); // word will get the value 5
    s.clear();
    println!("first word size: {word}, though s is cleared");
}

fn first_word_without_slices(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn solution_with_slice() {
    let s = String::from("hello world");
    let word = first_word_using_slices(&s);
    // s.clear();
    println!("first word: {word}, now s cant be cleared");
}

fn first_word_using_slices(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn array_slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
