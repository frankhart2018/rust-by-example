/*
    There are three types of structures:-

    1. Tuple structs:- Basically named tuples.

    2. Classic C structs.

    3. Unit structs:- Field-less, these are useful for generics.
*/

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with fields
struct Point {
    x: f32, 
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: &Rectangle) -> f32 {
    let Rectangle { 
        top_left: Point { x: top_left_x, y: top_left_y }, 
        bottom_right: Point { x: bottom_right_x, y: bottom_right_y } } = rectangle;

    (bottom_right_x - top_left_x) * (bottom_right_y - top_left_y)
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("Point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be same as `point.y` because we used the field from `point`
    println!("Second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: top_edge, y: left_edge } = point;

    let _rectangle = Rectangle {
        // Struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("Pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("Pair contains {:?} and {:?}", integer, decimal);

    println!("Area of rectangle = {}", rect_area(&_rectangle));
}
                