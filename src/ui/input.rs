use std::io::{self, Write};

pub fn read_input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

pub fn read_u32(message: &str) -> u32 {
    loop {
        let input = read_input(message);

        match input.parse::<u32>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

pub fn read_f64(message: &str) -> f64 {
    loop {
        let input = read_input(message);

        match input.parse::<f64>() {
            Ok(value) if value >= 0.0 => return value,
            _ => println!("Please enter a valid positive number."),
        }
    }
}