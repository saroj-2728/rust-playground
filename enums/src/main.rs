#![allow(unused)]

#[derive(Debug)]
enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32, i32),
}

impl Message {
    // Enums can also have methods
    fn call(&self) {
        println!("Fn called");
    }
}

fn main() {
    let private_address = IpAddressKind::V4(127, 0, 0, 1);

    let public_address = IpAddressKind::V6(String::from("::1"));

    println!("Private Address: {:#?}", private_address);
    println!("Public Address: {:#?}", public_address);

    println!();

    let m = Message::Write(String::from("monday"));
    m.call();
    dbg!(&m);

    println!();

    // The Option enum
    // made to remove the disadvantages of null values
    // enum Option<T> { // Included in the prelude, no need to bring it to the scope explicitly
    //     None,
    //     Some(T)
    // }

    let a_number = Some(5);
    let a_char = Some('s');

    let absent_number: Option<i32> = None; // Type need to be specified when assigning a variable with None

    let x: i16 = 32;
    let y: Option<i16> = Some(9);
    // let sum = x + y; // Error: different types
    // To perform operations, Option<T> first needs to be converted to type T

    /*
        In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value Option<T>. Then, when you use that value, you are required to explicitly handle the case when the value is null. Everywhere that a value has a type that isn’t an Option<T>, you can safely assume that the value isn’t null. This was a deliberate design decision for Rust to limit null’s pervasiveness and increase the safety of Rust code.
    */
}
