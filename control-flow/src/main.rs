#![allow(unused)]

#[derive(Debug)]
enum State {
    State1,
    State2,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

impl State {
    fn ls_value(&self, x: u16) -> bool {
        match self {
            State::State1 => x >= 1500,
            State::State2 => x < 1500
        }
    }
}



fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1, // arm, a pattern (here, Coin::Penny) and an expression (here just 1)
        Coin::Nickel => {
            println!("Some text here");
            5 // return value
        } // If multi lines to be written, curly braces, if curly braces, comma optional
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // when the pattern matches, state variable will bind to the value of that quarter's state
            println!("State: {:#?}", state);
            25
        } // all possible cases must be covered in the arms (Matches Are Exhaustive), else doesn't compile
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(value) => Some(value + 1),
    }
}



fn main() {
    println!("Hello, world!");
    value_in_cents(Coin::Quarter(State::State1));

    println!();

    let five = Some(5);
    let six = plus_one(five);
    println!("Incremented: {:#?}", six);

    let none = plus_one(None);
    println!("Not incremented: {:#?}", none);

    let dice_roll = 4;
    match dice_roll {
        1 => do_something(),
        2 => do_the_other_thing(),
        other => move_counts(other), // covers every other possible value, should be at last, if above catches everything and patterns after it will never be executed
        // OR
        // _ => some_fn() // if we do not want to use the value, _ matches any value but doesn't bind to that value
        // _ => (), to do nothing for other values, return unit value, empty tuple
    }
    
    fn do_something() {}
    fn do_the_other_thing() {}
    fn move_counts(count: u8) {}

    println!();

    let max_value = Some(3u8);
    match max_value {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    // can be made shorter with
    if let Some(max)  = max_value { // same as match, pattern is Some(max), expression max_value on the right
        // code below runs only when value matches the pattern
        println!("The maximum is configured to be {max}");
    } 
    else {
        // do_something; // same as _ => ... in match
    }

    println!();

    describe_the_value(Coin::Quarter(State::State1));

}




fn describe_the_value(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.ls_value(2000) {
            println!("Value too high");
            Some(String::from("Value too high"))
        }
        else {
            Some(String::from("Mid value"))
        }
    }
    else {
        None
    }
}

// improved version, return early if no match
fn describe_the_value_i1(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    }
    else {
        return None;
    };

    if state.ls_value(2000) {
        println!("Value too high");
        Some(String::from("Value too high"))
    }
    else {
        Some(String::from("Mid value"))
    }
}

// more improved version with let else, return if no match with let else
fn describe_the_value_i2(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else { // pattern on the left and an expression on the right similar to if let
        return None;
    };

    if state.ls_value(2000) {
        println!("Value too high");
        Some(String::from("Value too high"))
    }
    else {
        Some(String::from("Mid value"))
    }
}