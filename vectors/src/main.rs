fn main() {
    let _v: Vec<i32> = Vec::new();

    let _v = vec![1,2,3];

    let mut v = Vec::new(); // The numbers we place inside are all of type i32, and Rust infers this from the data, so we don’t need the Vec<i32> annotation.
    v.push(1); // Takes the type from this value, if this value is char it becomes Vec<char>
    v.push(2);
    v.push(3);

    for i in &v {
        println!("{i}");
    }

    // Reading elements of a vector: 2 ways
    let first = &v[0]; // By indexing
    println!("First element: {first}");

    let _second = v.get(1); // Using get method, returns an Option<&T>

    // For out of range values
    // let out = &v[10]; // Causes the program to panic
    let _out = v.get(10); // Returns None

    let mut v = vec![1,2,3,4,5];
    let _first = &v[0];
    v.push(6);
    // println!("First element: {first}"); // causes error cause this is an immutable ref and above first is a mutable ref to the vector v, both mut and immutable borrow not allowed in same scope

    // Iterating over the values of a vector
    for i in &v { // Immutable ref to each elem
        println!("{i}");
    }

    for i in &mut v {
        *i += 10;
    }


    // Vector can only store values of same type, but an enum can be used to store values of diff types, its still storing the single enum type but with mutltiple variants
    enum DTypes {
        Int(i32),
        Float(f32),
        Text(String)
    }

    let vec = vec![
        DTypes::Int(10),
        DTypes::Float(34.4),
        DTypes::Text(String::from("this is a string"))
    ];

}