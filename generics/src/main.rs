#![allow(unused)]
#![allow(non_snake_case)]

#[derive(Debug)]
struct Point2D<T> {
    // generics with struct,
    x: T,
    y: T,
    // here only allows both x and y to be of same type
}

#[derive(Debug)]
struct Point3D<T, U, V> {
    // allows x, y, z to be of different types
    x: T,
    y: U,
    z: V,
}

// On methods
impl<T> Point2D<T> {
    // <T> after impl indicates that <T> after Point2D is a generic type rather than a concrete type
    fn x(&self) -> &T {
        &self.x
    }
}

// We can also only define methods for a specific type rather than for any generic type
impl Point2D<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Same as struct for enums
// Eg. enum Option<T> { Some(T), None } and enum Result<T, E> { Ok(T), Err(E) }

fn main() {
    let num_list = vec![1, 234, 34, 12, 344, 33];
    println!("Largest num: {}", largest(&num_list));
    println!();

    let origin = Point2D { x: 0, y: 0 };
    let some_random_point: Point2D<f32> = Point2D { x: 34.3, y: 32.3 };
    println!("Coordinates of origin: {:#?}", origin);
    println!("Origin x: {}", origin.x);
    println!("Coordinates of other point: {:#?}", some_random_point);
    println!(
        "Distance from origin: {}",
        some_random_point.distance_from_origin()
    );
    println!();

    let a_3D_point = Point3D {
        x: 34,
        y: 3.5,
        z: 25.5,
    };
    println!("Coordinates of 3D point: {:#?}", a_3D_point);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
