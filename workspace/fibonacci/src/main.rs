use std::io::{Write, stdin, stdout};

fn main() {
    print!("Type a number: ");
    stdout().flush().unwrap();
    let mut n = String::new();
    stdin().read_line(&mut n).expect("Fail to read.");

    let n: i32 = n.trim().parse().expect("Should be a number.");
    let out: i32 = fibonacci(n);
    println!("The {} fibonacci number is {}.", n, out);
}

fn fibonacci(n: i32) -> i32 {
    if n <= 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
