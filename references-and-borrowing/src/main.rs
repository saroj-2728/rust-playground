fn main() {

    let str1 = String::from("This is a string.");

    // Action of creating a reference is borrowing.
    let len1 = calc_str_length(&str1);
    println!("Length of string: {str1} = {len1}");

    // Mutable reference
    let mut str2 = String::from("This is another string. ");
    let len2 = mutable_reference(&mut str2);
    println!("Length of modified string: {str2} = {len2}");

    // Restriction of &mut: if you have a mutable reference to a value, you can have no other references to that value
    // let mut str3 = String::from("Hyalo");
    // let r1 = &mut str3;
    // let r2 = &mut str3; // throws errors: cannot borrow str3 as mutable more than once at a time
    // println!("Ref 1: {r1}, Ref 2: {r2}");
    // println!("Ref 1: {r1}"); // if str3 is mutable borrowed by r2 after usage of r1, then it works as usual

    // let mut str4 = String::from("Another string");
    // let r1 = &str4; // no problem
    // let r2 = &str4; // no problem
    // let r3 = &mut str4;
    // println!("{}, {} and {}", r1, r2, r3); // err: can't borrow str4 as mutable as it is also borrowed as immutable

    // A reference’s scope starts from where it is introduced and continues through the last time that reference is used
    // This works cause last usage of immutable reference is in println!, before mutable reference is introduced.
    let mut str5 = String::from("String");
    let r1 = &str5;
    let r2 = &str5;

    println!("R1: {r1}, R2: {r2}");
    
    let r3 = &mut str5;
    println!("R3: {r3}");


    // Dangling Refernces, prevented by default by rust compiler
    // eg:
    // let reference_to_nothing = dangle();

}

fn calc_str_length(str: &String) -> usize {
    // str.push_str("string"); // throws error: can't borrow *str as mutable
    str.len()
}

// Mutable references to modify the borrowed value
fn mutable_reference(str: &mut String) -> usize {
    str.push_str("Appended string");    
    str.len()
}

// throws errors
// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

// The solution here is to return the String directly:

// fn no_dangle() -> String {
//     let s = String::from("hello");

//     s
// }  