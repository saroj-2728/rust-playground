fn main() {
    // All variables by default immutable, to make mutable, add mut
    let mut x = 5;
    println!("Value of x = {x}");
    x = 6; // Throws error if mut is not used ahead of x
    println!("After increment x = {x}\n");

    // Constants can't be set the result of any operation
    // const A_CONST = x + 3; // Throws an error
    // Type should be mentioned and const variable must be uppercase
    const A_CONST_VALUE: u32 = 60 * 60 * 3;
    println!("Const value = {A_CONST_VALUE}\n");

    // Shadowing
    let x = 10; // Shadows the previous x

    {
     let x = x + 122; // Shadows the previous x
     println!("Value of x inside the inner scope: {x}");
     // When the scope ends the inner scope shadowing ends
    }
    println!("Value of x after being shadowed: {x}\n");

    let spaces = "  ";
    let spaces = spaces.len(); // Allowed coz of shadowing
    
    // If we try to do this
    // let mut spaces = "  ";
    // spaces = spaces.len();
    // An error is thrown as mismatched type, as it is not allowed to mutate a variable's type
    
    println!("No. of spaces: {spaces}");
}
