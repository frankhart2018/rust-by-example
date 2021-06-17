/*
    To convert any type to String is as simple as implementing the 
    ToString trait for the type

    Rather than doing so directly, we should implement the fmt::Display
    trait which automagically provides ToString and also allows printing the type
*/

use std::fmt;

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // Parsing a string converts string into that type
    // so long as FromStr trait is implemented for that type

    let parsed: i32 = "5".parse().unwrap();

    // .method<type>:() is called turbofish syntax
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}          