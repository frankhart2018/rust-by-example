#[allow(unused_code)]

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

use crate::List::*;

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),

    // Nil: A node that signifies the end of linked list
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        // `self` has to be matched, because the behaviour of this method
        // depends on the variant of `self`
        // `self` has the `&List`, and `*self` has type `List`, matching on a 
        //concrete type `T` is preferred over a match on a reference `&T`
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("")
            },
        }
    }
}

fn main() {
    println!("Zero is {}", Number::Zero as i32);
    println!("One is {}", Number::One as i32);

    println!("Roses are #{:06x}", Color::Red as i32);
    println!("Violets are #{:06x}", Color::Blue as i32);

    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("Linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
                