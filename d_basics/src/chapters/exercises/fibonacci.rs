pub fn fibonacci() {
    println!("\n\n--- Fibonacci Series ---\n");

    for i in 0..15 {
        println!("fibonacci ({}) => {}", i, fib(i));
    }
}

fn fib(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}
