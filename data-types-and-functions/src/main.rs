fn main() {

    // let num = "42".parse().expect("Not a number"); // Type required
    let num: u32 = "42".parse().expect("Not a number");
    println!("Num: {num}");

    // Scalar Types
        // Integers, i  = signed, u = unsigned, bits variation: 8, 16, 32, 64, 128, arch i.e usize and isize (acc to archi of the system)
        let another_num: usize = 32; 
        print!("Num with type usize: {another_num}\n");

        // Floats, f32 and f64
        let a_float = 23.2;
        let another_float: f32 = 23.3;
        print!("Floats: {a_float} and {another_float}\n");

        // Booleans
        let a_bool = false;
        let another_bool: bool = true;
        print!("Booleans: {a_bool} and {another_bool}\n");

        // Chars, under single quotes, str under double quoutes
        let a_char = 'd';
        let a_string = "A string";
        print!("Char: '{a_char}', String: \"{a_string}\"\n");

    
    // Compound Types
        // Tuple
            //  fixed length, each element's type need not be same
            let tup = (1, 23.3, 'f');
            let _tup: (i64, f32, &str) = (12, 23.3, "");
        
            // Destructuring
            let (a, b, c) = tup;
            print!("Values: {a}, {b}, {c}\n");

            // Can also be accessed with . and then index
            let x = tup.1;
            print!("X: {x}\n");
        
        // Array
            // Every element must have the same type, fixed length
            let an_array = [5;4]; // Length 4 with all elements 5
            let another_array = [1,2,3,4,5];

            // Accessing elements
            let first_of_first = an_array[0];
            let second_of_second  = another_array[1]; // if index out of bounds, panics
            print!("First: {}, Second: {}\n", first_of_first, second_of_second);
            
            let x = another_function(55);
            print!("Returned value from another_function: {}\n", x);
}


fn another_function(x: i32) -> i32{
    // Statements -> do not return a value
    // Expressions -> return a value

    let _y = 5; // A statement
    // let x = (let y = 54); // Throws err as let y = 54 a statement, no value returned, so nothing to assign to let x
    
    // Fn definitions are also statements

    let y = {
        let x = 5;
        x+5 // no semicolon, so x+5 returned to y, so y = 10
    };
    println!("Y: {y}\n"); // Gives Y: 10

    print!("Received argument: {x}\n");
    println!("Five: {}", five());

    5 // if no semicolon returns 5, same as return 5; or return 5
}

fn five() -> i32 {5} // Returns 5