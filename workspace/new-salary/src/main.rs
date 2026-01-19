use std::io;

fn new_salary(position: String, salary: f32) -> f32 {
    match position.trim() {
        "Entry" => salary * 1.10,
        "Mid" => salary * 1.15,
        "Senior" => salary * 1.20,
        _ => -1.,
    }
}

fn main() {
    println!("What is your salary:");
    let mut salary = String::new();
    io::stdin()
        .read_line(&mut salary)
        .expect("Fail to read the salary.");
    let salary: f32 = salary.trim().parse().expect("Only numbers.");

    println!("What is your level position:");
    let mut position = String::new();
    io::stdin()
        .read_line(&mut position)
        .expect("Fail to read the position");

    println!("The new salary is {}", new_salary(position, salary));
}
