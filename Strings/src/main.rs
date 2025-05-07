fn main() {
    // Creating
    let _s = String::new(); // empty string

    let _s = String::from("Something"); // with some initial conten
    let _s = "Something".to_string();

    // Updating
    let mut s = "Something".to_string();
    s.push_str(" Amazing!"); // takes a str slice (borrowed) as parameter (here " Amazing")
    s.push('s'); // to add a character
    println!("String: {s}");

    // Concatenating with +
    let st = String::from("here");
    let ss = s + &st; //The reason s1 is no longer valid after the addition, and the reason we used a reference to s2, has to do with the signature of the method that’s called when we use the + operator. The + operator uses the add method, whose signature looks something like this: fn add(self, s: &str) -> String {

    // println!("String: {s}"); // throws errors borrow of moved value, cause s's value moved in above line
    println!("String: {ss}");

    // for more strings
    let s1 = String::from("Hello");
    let s2 = String::from(" there!");
    let s3 = String::from(" hey");
    // let s4 = s1 + &s2 + " Oh" + &s3;
    // or
    let s4 = format!("{s1}{s2} Oh{s3}"); // Works like println!, uses refs so doesn't take any ownership
    println!("String: {s4}");

    let _s = String::from("Some string");
    // let first_char = s[0]; // Error, String can't be indexed by integer
    // REASON: The answer, then, is that to avoid returning an unexpected value and causing bugs that might not be discovered immediately, Rust doesn’t compile this code at all and prevents misunderstandings early in the development process.

    // Slicing Strings
    let hello = "Здравствуйте"; // Cyrillic letters
    let _s = &hello[0..4]; // s gets Зд, if we use [0..1] throws error, cause each character above is of 2 bytes and [0..1] would not be a char boundary, i.e it would be inside З

    // Iterating over Strings
    // Calling chars on “Зд” separates out and returns two values of type char
    for c in "Зд".chars() {
        println!("{c}");
    }

    // the bytes method returns each raw byte
    for c in "Зд".bytes() {
        println!("{c}");
    }
}
