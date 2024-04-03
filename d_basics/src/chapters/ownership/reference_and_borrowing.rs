pub fn reference_and_borrowing() {
    return_value_using_reference();
    mutable_reference();
    multiple_mutable_reference();
    both_mutable_and_immutable_reference();
    dangling_reference();
}

fn return_value_using_reference() {
    let s1 = String::from("hello");

    let len = calculate_length_2(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length_2(s: &String) -> usize {
    s.len()
}

fn mutable_reference() {
    let s = String::from("hello");
    change_1(&s);

    let mut s = String::from("hello");
    change_2(&mut s);
}

fn change_1(_some_string: &String) {
    // some_string.push_str(", world"); // can't mutate
}

fn change_2(some_string: &mut String) {
    some_string.push_str(", world");
}

fn multiple_mutable_reference() {
    let mut s = String::from("hello");
    let _r1 = &mut s;
    // let r2 = &mut s; // cant have two mutable reference
    // println!("{}, {}", r1, r2);

    let mut s = String::from("hello");

    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let _r2 = &mut s;
}

fn both_mutable_and_immutable_reference() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // PROBLEM: cant have both mutable and immutable
                 // println!("{}, {}, and {}", r1, r2, r3);
    println!("both immutable reference end here: {} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("now mutable reference can be used: {}", r3);
}

fn dangling_reference() {
    // let reference_to_nothing = dangle();
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } s is out of scope, so nothing exist
