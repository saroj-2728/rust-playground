#![allow(unused)]

use std::fmt::Display;
// Lifetime Annotations in Struct Definitions
struct Sen<'a> {
    text: &'a str,
}

// Lifetime Annotations in Method Definitions
impl<'a> Sen<'a> {
    fn some_method(&self) -> i32 {
        // 1st and 2nd elision rule and OK
        45
    }
}

impl<'a> Sen<'a> {
    fn another_method(&self, s: &str) -> &str {
        // 1st and 3rd elision rule and OK
        println!("{} and {s}", self.text);
        self.text
    }
}

fn main() {
    let str1 = String::from("String 1");
    let result;
    {
        let str2 = String::from("This is string 2");
        result = longest_str(str1.as_str(), str2.as_str()); // This doesn't work cause the smaller lifetime is of str2 which is valid only inside inner scope and we are trying to print result outside inner scope
    }
    // println!("Longest string: {result}");

    let some_string = "some string here";
    let a_struct = Sen { text: some_string };

    // The Static Lifetime
    // The reference lives for the entire duration of the program
    let static_stri = "This is a string having static lifetime";
}

// 'a lifetime generic
// this fn ensures that the returned reference will be valid as long as both the parameters are valid
// i.e  lifetime of the reference returned by the fn is the same as the smaller of the lifetimes of the values referred to by the function arguments.
fn longest_str<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() { str1 } else { str2 }
}

// When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.
// Eg. the below fn doesn't compile
// fn longest_str1<'a>(str1: &str, str2: &str) -> &'a str {
//     let result = String::from("Some string here!");
//     result.as_str()
// }

// Lifetime Elision
// A set of particular cases that the compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.
// 3 Rules:

// 1. The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference.
// Eg: fn foo<'a>(x: &'a i32), fn foo<'a, 'b>(x: &'a i32, y: &'b i32) and so on

// 2. The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

// 3. The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.

// where Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.

// If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error.

// This fn works without any errors
// the 1st rule and 2nd rule get applied and fn is considered as
// fn first_word<'a>(x: &'a str) -> &'a str {...}
// and after applying the 3 rules, all the references in the fn signature now have lifetimes and thus no errors
fn first_word(x: &str) -> &str {
    let bytes = x.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &x[0..i];
        }
    }

    &x[..]
}

// But in above longest_str fn, 2 parameters with 2 diff lifetimes, so after applying 3 rules, the compiler doesn't know what will be the lifetime of the returned reference, so error

// Generic Type Parameters, Trait Bounds, and Lifetimes Together
fn longest_with_an_announcement<'a, T>(
    // lifetime parameters must be declared prior to type and const parameters
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() { x } else { y }
}