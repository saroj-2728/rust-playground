struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {

    let user1 = User {
        active: true,
        username: String::from("saroj"),
        email: String::from("user@example.com"),
        sign_in_count: 435,
    };

    println!("User's username: {}", user1.username);


    let mut user2 = User {
        active: true,
        username: String::from("hyalo"),
        email: String::from("user@example1.com"),
        sign_in_count: 43
    };

    user2.active = false;
    println!("User2's active: {}", user2.active);


    let user3 = build_user(String::from("e@gmail.com"), String::from("username_here"));
    println!("User3's sign in count: {}, email: {}", user3.sign_in_count, user3.email );


    let _user4 = User {
        active: false,
        ..user3 // active = false, others same as that of user3, called as struct update syntax, must come at the last position
    };
    // user3 can no longer be used, caused its data have been moved to user4
    // println!("User3's email: {}", user3.email); // throws error, borrow of moved value as email has String type which do not implement Copy trait


    let _user5 = User {
        email: String::from("email@gmali.com"),
        username: String::from("user5"),
        ..user2
    };
    // user2 can be used further cause only actice and sign_in_count fields are used from user2 which implement Copy trait
    println!("User2's email: {}", user2.email); // no errors

    
    

    // Tuple Structs
    struct Point3D(i32, i32, i32); // no names associated with their fields only types

    let origin = Point3D(0, 0, 0);
    println!("Origin: ({}, {}, {})", origin.0, origin.1, origin.2);

    let Point3D(x, y, z) = origin; // destructuring from a tuple struct
    println!("x: {}, y: {}, z: {}", x, y, z);



    // Unit-like structs, (() = unit type, expressions implicitly return unit value if they don't return any other value)
    struct AlwaysEqual;
    let _sub = AlwaysEqual;




    // Ownership of Struct Data
    struct User1 {
        active: bool,
        email: &str,
        username: &str,
        sign_in_count: i32
    } // doesn't work as it doesn't specify lifetime





}


fn build_user (email: String, username: String) -> User {
    User {
        active: true,
        username: username, // or username,
        email: email,       // email, // field init shorthand
        sign_in_count: 1, 
    }
}