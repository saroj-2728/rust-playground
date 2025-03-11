fn main() {

    let str1 = String::from("hello world!");
    // let word = first_word(&str1);
    // str1.clear();
    // println!("Index: {word}");


    let _len = str1.len();
    let hello = &str1[0..5]; // .. = rust's range syntax
    // let hello = &str1[..5]; // valid, first value can be omitted if 0
    let world = &str1[6..11];  // world is a slice that contains a pointer to the byte at index 6 of str1 witha a length value of 5
    // let world = &str1[6..]; // is eq. to let world = &str1[6.._len] // if last value omitted, then upto the last index
    // let world = &str1[..] // eq. to let world = &str1[0.._len] whole string as a slice
    println!("World: {world}");


    // first_word works on slices of String's whether partial or whole and slices of string literals too whether partial or whole, if it accepts &str instead of &String
    let first_word_s = first_word(&str1);
    // let first_word_s = first_word(&str1[0..8]); // also works on this
    println!("First word: {first_word_s}");


    let string_literal = "This is a string";
    let first_word_l = first_word(string_literal); // no ref req as already a ref, immutable ref
    // let first_word_l = first_word(&string_literal[..7]); // also works on this
    println!("First word of string literal: {first_word_l}");

    // array can also be sliced
    let a = [1,3,3,5,52,];
    let sliced_array = &a[2..4];
    
    for elem in sliced_array {
        println!("Element: {elem}");
    }

    assert_eq!(sliced_array, &[3,5])

}

fn _first_word(str: &String) -> usize {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    str.len()
}

// can be made to work on &str and &String both by using (s: &str) instead of (s: &String) in parameters
fn first_word(s: &str) -> &str { // works on 
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}