use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error trying to read from stdin");
    let n: u32 = n.trim().parse().expect("Please provide a positive integer");
    let result = fibonacci(n);
    println!("The fibonacci number at index {} is {}.", n, result);
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
