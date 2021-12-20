use fancy_regex::Regex;
use std::{collections::HashMap, io};

fn main() {
    let mut employees: HashMap<String, String> = HashMap::new();
    loop {
        println!("Please enter the phrase! (Empty to proceed)");
        let input = {
            let mut buffer = String::new();
            io::stdin()
                .read_line(&mut buffer)
                .expect("Error reading from stdin");
            buffer
        };
        let trim = input.trim();
        if trim.is_empty() {
            break;
        }
        let name = get_name(trim);
        let function = get_function(trim);
        employees
            .entry(name.to_owned())
            .or_insert(function.to_owned());
    }

    loop {
        println!("Please enter a name to search! (Empty to leave)");
        let input = {
            let mut buffer = String::new();
            io::stdin()
                .read_line(&mut buffer)
                .expect("Error reading from stdin");
            buffer
        };
        let trim = input.trim();
        if trim.is_empty() {
            break;
        }
        match employees.get(trim) {
            Some(function) => println!("{} function is {}.", trim, function),
            None => println!("Couldn't find {} on the databse!", trim)
        }
    }
}

fn get_name(s: &str) -> &str {
    let words = Regex::new(r"(?i)(?<=Add )\w+(?= to)").unwrap();
    let captures = words.captures(s).expect("Error executing regex.");
    match captures {
        // Safe to unwrap because if it's Some, it has at least one capture
        Some(cap) => cap.get(0).unwrap().as_str(),
        None => "",
    }
}

fn get_function(s: &str) -> &str {
    let words = Regex::new(r"(?i)(?<=to )\w+[^\n\t ]").unwrap();
    let captures = words.captures(s).expect("Error executing regex.");
    match captures {
        // Safe to unwrap because if it's Some, it has at least one capture
        Some(cap) => cap.get(0).unwrap().as_str(),
        None => "",
    }
}
