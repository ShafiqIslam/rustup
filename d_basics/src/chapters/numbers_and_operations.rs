pub fn numbers_and_operations() {
    println!("\n\n--- Numeric Data types And Operations ---\n");

    println!("Min i8: {}", i8::MIN);
    println!("Min i32: {}", i32::MIN);
    println!("Min i64: {}", i64::MIN);
    println!("Min i128: {}", i128::MIN);
    println!("Min isize: {}", isize::MIN);
    println!("Max u8: {}", u8::MAX);
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);

    let double = 1.111111111111111;
    let single: f32 = 1.111111111111111;

    println!("single precision: {}", single + 0.111111111111111);
    println!("double precision: {}", double + 0.111111111111111);

    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    println!("56.7 / 32.2 = {quotient}, -5 / 3: {truncated}");
    let _remainder = 43 % 5;
}
