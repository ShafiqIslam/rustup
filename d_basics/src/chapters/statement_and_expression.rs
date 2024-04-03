pub fn statement_and_expression() {
    println!("\n\n--- Expression and statement ---\n");
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
