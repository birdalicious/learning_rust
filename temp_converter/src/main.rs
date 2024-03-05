use std::{io, str::FromStr};

fn main() {
    println!("Temperature Converter");

    println!("What is your temperature measured in?");
    println!("1. Fahrenheit");
    println!("2. Celsius");

    let option = loop {
        let option: i32 = take_user_input("Pick option:");

        match option {
            1 => break Units::Fahrenheit,
            2 => break Units::Celsius,
            _ => (),
        }

        println!("Pick either 1 or 2");
    };

    let temp: f64 = take_user_input(&format!("Enter temperature in {:?}:", option));

    let converted = match option {
        Units::Fahrenheit => (temp - 32.0)/1.8,
        Units::Celsius => temp * 1.8 + 32.0,
    };

    println!("Converted to {}", converted);
}

#[derive(Debug)]
enum Units {
    Fahrenheit,
    Celsius,
}

fn take_user_input<T: FromStr>(prompt: &str) -> T {
    let mut input = String::new();

    loop {
        println!("{}", prompt);

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");

        match input.trim().parse() {
            Ok(val) => break val,
            Err(_) => {
                println!("please enter a number");
                continue;
            },
        }
    }
}