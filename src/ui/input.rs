use std::io::{self, Write};
use std::str::FromStr;

pub fn read_input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

pub fn read_number<T, F>(message: &str, error: &str, accept: F) -> T
where
    T: FromStr,
    F: Fn(&T) -> bool,
{
    loop {
        let input = read_input(message);

        match input.parse::<T>() {
            Ok(value) if accept(&value) => return value,
            _ => println!("{}", error),
        }
    }
}

pub fn read_u32(message: &str) -> u32 {
    read_number(message, "Please enter a valid number.", |_| true)
}

pub fn read_f64(message: &str) -> f64 {
    read_number(
        message,
        "Please enter a valid positive number.",
        |value| *value >= 0.0,
    )
}