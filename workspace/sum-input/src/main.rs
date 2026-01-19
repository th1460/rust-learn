use std::io;

fn main() {
    let mut sum: i32 = 0;
    loop {
        println!("Type a number:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Fail to read.");
        let input: i32 = input.trim().parse().expect("Should be a number.");
        sum += input;
        println!("Sum is {}", sum);
        if input == 0 {
            println!("Bye!");
            break;
        }
    }
}
