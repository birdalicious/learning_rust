use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Enter a command (e.g., 'Add Sally to Engineering' or 'List Engineering'):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Should be able to read line");

        let input = input.trim();
        if input.is_empty() {
            println!("Please enter a valid command");
            continue;
        }

        let command = input
            .split_ascii_whitespace()
            .next()
            .unwrap()
            .to_ascii_lowercase();

        match command.as_str() {
            "add" => add_command(&mut departments, &input),
            "list" => list_command(&departments, &input),
            "quit" => break,
            _ => {
                println!("Invalid command. Please use 'Add' or 'List'.");
            }
        }
    }
}

fn add_command(departments: &mut HashMap<String, Vec<String>>, command: &str) {
    static ADD_COMMAND_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"add\s(.+)\sto\s(.+)").expect("regex should be valid"));

    let Some(caps) = ADD_COMMAND_REGEX.captures(command) else {
        println!("Add command needs to be the format of 'Add {{name}} to {{department}}'");
        return;
    };
    let name = String::from(&caps[1]);
    let department = String::from(&caps[2]);

    let names = departments.entry(department).or_insert(Vec::new());
    names.push(name);
}

fn list_command(departments: &HashMap<String, Vec<String>>, command: &str) {
    static LIST_COMMAND_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"list\s(.+)").expect("regex should be valid"));

    let Some(caps) = LIST_COMMAND_REGEX.captures(command) else {
        println!("List command needs to be the format 'List {{department}}'");
        return;
    };

    let department = String::from(&caps[1]);

    if department.to_ascii_lowercase() == "departments" {
        for department in departments.keys() {
            println!("{}", department);
        }
        return;
    }

    let Some(departments) = departments.get(&department) else {
        println!("No department called {department}");
        return;
    };

    for department in departments.iter() {
        println!("{}", department);
    }
}
