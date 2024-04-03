pub fn tuples() {
    println!("\n\n--- Tuples ---\n");
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of 2nd position of tuple is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;
}
