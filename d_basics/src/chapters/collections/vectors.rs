pub fn vectors() {
    println!("\n\n--- Vectors ---\n");

    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);

    let third: &i32 = &v[2]; // program will panic if index does not exist
    println!("The third element is {third}");

    match v.get(2) {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6); // both mutable and immutable reference cant be used
    println!("The first element is: {first}");
}
