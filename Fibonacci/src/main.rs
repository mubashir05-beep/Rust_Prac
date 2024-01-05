// Generate the nth Fibonacci number.
use std::io;

fn main() {
    println!("Enter the nth value:");

    let mut nth_number = String::new();

    io::stdin()
        .read_line(&mut nth_number)
        .expect("Failed to read!");

    let nth_number: u32 = nth_number.trim().parse().expect("User Entered String!");

    let fibonacci_result = generate_nth_fibonacci(nth_number);

    println!(
        "The {}th Fibonacci number is: {}",
        nth_number, fibonacci_result
    );
}
fn generate_nth_fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}
