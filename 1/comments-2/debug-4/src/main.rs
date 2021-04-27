
fn main() {
    #[derive(Debug)]
    struct DebugPrintable(i32);

    // Derive the `fmt::Debug` implementation for `Structure`. `Structure`
    // is a structure which contains a single `i32`.
    #[derive(Debug)]
    struct Structure(i32);

    // Put a `structure` inside of the structure `Deep`. Make it printable
    // also
    #[derive(Debug)]
    struct Deep(Structure);

    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");
    
    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look.
    println!("Now {:?} will print!", Deep(Structure(7))); 

    // Pretty printing structures
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:#?}", peter);
}
                