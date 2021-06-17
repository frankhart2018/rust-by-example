/*
    The From trait allows for a type to define how to create itself 
    from another type, hence providing a very simple mechanism for converting
    between several types

    There are numerous implementations of this trait within the standard library 
    for conversion of primitive and common types

    The Into trait is simply reciprocal of the From trait

    That is, if we have implemented the From trait for our type,
    Into will call it when necessary

    Using the Into trait will typically require specification of the type 
    to convert into as the compiler is unable to determine this most of the time

    However, this is a small trade-off considering we get the functionality for free
*/

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let my_str = "hello";
    let _my_string = String::from(my_str);

    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
                