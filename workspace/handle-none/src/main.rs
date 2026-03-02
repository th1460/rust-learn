fn plus_one(input: Option<i32>) -> i32 {
    match input {
        Some(value) => {
            // Return the unwrapped 'value' of type i32
            value + 1
        },
        None => {
            // Handle the absence of a value by returning a default i32
            0
        }
    }
}

fn main() {
    let six = plus_one(Some(5));
    println!("{}", six);
    let none = plus_one(None);
    println!("{}", none);
}

