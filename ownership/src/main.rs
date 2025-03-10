fn main() {

    let mut s = String::from("A mutable string. ");
    s.push_str("Appended one. ");
    println!("{s}");

    let s1 = s.clone();
    println!("{s}  {s1}");

    let x = 4;
    let y = x;
    println!("x: {x}, y: {y}");

    takes_ownership(&s); 
    println!("{s}"); // If no reference used, throws error: borrow of moved value (cause s's value moved into the func takes_ownership, owner changed)
    
    makes_copy(x);
    println!("{x}"); // Nothing needed cause i32 implements the Copy trait, x doesn't move into the function

    let test_str = gives_ownership();
    println!("{test_str}");

    let str1 = takes_and_give_back_ownership(test_str);
    println!("{str1}");

    let str2 = String::from("Another test string.");

    let a_tuple = return_multiple_values_using_tuple(str2);
    let (s, length) = a_tuple;
    println!("Returned tuple values: {s}, {length}");
    
}


fn takes_ownership(a_string: &String){
    println!("{a_string}");
}


fn makes_copy(num: i32){
    println!("{num}");
}

fn gives_ownership() -> String{
    let str = String::from("A test string");
    str
}

fn takes_and_give_back_ownership(str: String) -> String{
    str
}

fn return_multiple_values_using_tuple(s: String) -> (String, usize){
    let length = s.len();

    (s, length) // To return multiple values without using references
}