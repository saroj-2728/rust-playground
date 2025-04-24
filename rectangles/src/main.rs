// fn main() {
//     let length = 29;
//     let breadth = 1;

//     println!(
//         "The area of the rectangle is {} square pixels",
//         area_of_rectangle(length, breadth)
//     )
// }

// fn area_of_rectangle(l: u32, b: u32) -> u32 {
//     l * b
// }

// Using Tuple
// fn main(){
//     let rect1 = (12,12);

//     println!(
//         "The area of rectangle is {} square pixels.",
//         area_of_rectangle(rect1)
//     );

// }

// fn area_of_rectangle(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Using Structs
#[derive(Debug)] // Explicitly opting in to make printing debugging info functionality available for the struct // should be just above the struct
struct Rectangle {
    length: u32,
    breadth: u32,
}

// Fns defined within an impl block -> Associated fns
impl Rectangle { // Implementation
    fn area(&self) -> u32 { // a method of struct Rectangle, 1st parameter always self
        self.length * self.breadth
    }

    // a getter
    fn length(&self) -> bool { // Can have a method of the same name as that of the struct's fields
        self.length > 0
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        self.length > rect2.length && self.breadth > rect2.breadth
    }

    // fns that don't have self as 1st parameter -> not methods, eg: String::from
    // Often used for constructors
    fn square(size: u32) -> Self { // here Self is the alias for Rectangle, the type that appears after impl 
        Self {
            length: size,
            breadth: size
        }
    }
} // Each struct can have multiple impl blocks

fn main() {
    let rect1 = Rectangle {
        length: 12,
        breadth: 9,
    };

    println!("{:#?}", rect1); // Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug. The Debug trait enables us to print our struct // Additional # for pretty-print
    println!(
        "Area of rectangle = {} square pixels",
         area(&rect1)
    );

    // or using method

    println!(
        "Area of rect1 = {}",
        rect1.area()
    );

    println!(
        "Rectangle's length > 0: {}",
        rect1.length()
    );

    let rect3 = Rectangle {
        length: 1,
        breadth: 4
    };

    println!(
        "Can rect1 hold rect3? -> {}",
        if rect1.can_hold(&rect3) { String::from("Yes") } else { String::from("No") }
    );

    let sq = Rectangle::square(2);    // Calling a constructor, similar to String::from("Something")
    // :: syntax used for associated fns and namespaces created by modules


    println!();

    let scale = 2;
    let rect2 = Rectangle {
        length: dbg!(4 * scale), // Can be used like this coz dbg! takes and returns ownership, same as 4 * scale
        breadth: 2,
    };

    // Usinng dbg! macro to display which prints to standard error console stream (stderr) as opposed to println! macro which prints to standard output console stream (stdout)
    dbg!(&rect2); // here we don't want dbg! to take ownership so we use reference
    // Also shows which file and which line was this macro called from
}

fn area(rect: &Rectangle) -> u32 {
    rect.length * rect.breadth
}
