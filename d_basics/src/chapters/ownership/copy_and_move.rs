pub fn copy_and_move() {
    copy_move_clone();
    copy_move_with_functions();
    return_ownership_from_function();
    return_value_and_ownership_using_tuple();
}

fn copy_move_clone() {
    let x = 5;
    let _y = x; // this is copied as it's using stack memory

    let s1 = String::from("hello");
    let _s2 = s1; // this is moved as it's using heap
                  // println!("{}", s1); // as its moved, s1 is no longer available

    let s1 = String::from("hello");
    let s3 = s1.clone(); // a complete copy is created
    println!("String cloned = {}", s3);
}

fn copy_move_with_functions() {
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s moves to function
                        // println!("{}", s); // s is no longer available

    let x = 5; // x comes into scope
    makes_copy(x); // x move to function but i32 is Copy
    println!("i32 is copied, not moved = {}", x); // so its valid here
}

fn takes_ownership(s: String) {
    // s comes into scope
    println!("takes_ownership: {}", s);
} // s goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("makes_copy: {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn return_ownership_from_function() {
    let _s1 = gives_ownership(); // gives_ownership moves its return value into s1
    let s2 = String::from("hello"); // s2 comes into scope
    let _s3 = takes_and_gives_back(s2);
    // s2 is moved into takes_and_gives_back, which also moves its return value into s3
} // s1, s3 goes out of scope and is dropped. s2 was moved, so nothing happens.

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn return_value_and_ownership_using_tuple() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length_1(s1);
    // calculate_length returns both ownership and some value. because s1 is moved and not avaiable.

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length_1(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
