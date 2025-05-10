#![allow(unused)]
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_result = File::open("hello.txt");

    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(er) => panic!("Error creating the file: {:#?}", er),
            },
            other_error => panic!("Error while opening file: {:#?}", other_error),
        },
    };

    // .unwrap()
    let file = File::open("hlo.txt").unwrap(); // If result is Ok variant, unwrap() returns the value inside Ok, if Err variant, unwrap calls the panic! macro

    let file = File::open("hlo.txt").expect("hello.txt file not found!"); // Same as unwrap, just lets us write the message for panic! macro

    let file = File::open("hello.txt")?; 
    Ok(())
    // Above 2 lines work if main's return type is changed as above otherwise,
    // Throw error, cause the ? operator can only be used in a fn that returns either Result or Option
    // ? works the same way with Option as it does with Result
    // if Some(val) => returns val to the expression
    // if None => returns None early out of the entire fn
}

// ? eg. with Option
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()

    // lines() method returns an iterator over the lines in the string, next() gives the first value of the iterator, if line's empty next retuns None and the fn returns with None cause of ?, if not empty next retuns a Some value containing string which is extracted by ?. the chars() returns iterator over the characters, and lastly last() to get the last character
}

// Propagating Errors: Instead of handling errors within fn itself, returning the error to the calling code
fn read_username_from_file() -> Result<String, io::Error> {
    let file_result = File::open("hello.txt");

    let mut username_from_file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_from_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Using ? operator
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?; // ? at the end
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)

    // ? does the same thing as match above, if the value of Result is an Ok, value inside Ok gets returned to the expression (here variable "file"), if Err variant, the Err will be returned from the whole fn
    // Only diff, the ? operator goes through the 'from' fn which converts the received error type to the error type defined in the return type of fn
}

// The fn can be made even simpler by chaining the method calls after the ?
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Or just
fn read_username_from_file4() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}
// Any other way to make this even shorter? 😑😑