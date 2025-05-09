// Qns from "https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html"
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    // Q1
    let mut nums_list = vec![3, 4, 1, 5, 2];
    let median = find_median(&mut nums_list);
    println!("Median of {nums_list:?}: {median}");
    println!();

    // Q2
    let mut word = String::from("something");
    println!("In pig-latin, {word} = {}", convert_to_pig_latin(&mut word));
    println!();

    // Q3
    let mut users_map: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut department_name = String::new();
        print!("Enter the department name (Press Enter to skip): ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut department_name)
            .expect("Failed to read line!");
        println!();

        if department_name.trim().is_empty() {
            break;
        } else {
            println!("Enter the names of the employees (Press Enter to skip): ");

            let mut employee_names: Vec<String> = Vec::new();
            loop {
                let mut employee_name = String::new();
                io::stdin()
                    .read_line(&mut employee_name)
                    .expect("Failed to read line!");

                if employee_name.trim().is_empty() {
                    break;
                } else {
                    employee_names.push(employee_name.trim().to_string());
                }
            }

            users_map.insert(department_name.trim().to_string(), employee_names);
        }
    }

    print!(
        "What do you wanna see? \n1. All employees in the company. \n2. Employees of a specific department"
    );
    print!("\n\nEnter your choice: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line!");

    let choice: Option<u32> = match choice.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Please enter a valid number!");
            None
        }
    };

    match choice {
        Some(num) => {
            if num == 1 {
                println!("All Departments with Employee names: ");
                println!("{users_map:#?}");
            }
            else {
                print!("\nEnter department name: ");
                io::stdout().flush().unwrap();

                let mut department_name = String::new();
                io::stdin()
                    .read_line(&mut department_name)
                    .expect("Failed to read line!");

                if users_map.contains_key(&department_name.trim().to_string()) {
                    println!("Employees of department {}:", department_name.trim());
                    println!("{:#?}", users_map[&department_name.trim().to_string()]);
                }
            }
        }
        None => println!("Do something"),
    }
}

// Q1
fn find_median(list: &mut Vec<i32>) -> i32 {
    list.sort();
    if list.len() % 2 == 0 {
        list[list.len() / 2]
    } else {
        list[(list.len() - 1) / 2]
    }
}

// Q2
fn convert_to_pig_latin(text: &mut String) -> String {
    let first_char = &text[0..1];
    match first_char {
        "a" | "e" | "i" | "o" | "u" => format!("{text}-hay").to_string(),
        _ => {
            let end = format!("-{first_char}ay");
            let mut final_string = text[1..].to_string();
            final_string.push_str(&end);
            final_string.to_string()
        }
    }
}
