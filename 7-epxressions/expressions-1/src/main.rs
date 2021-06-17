/*
    The two most common statements in Rust are binding and expressions 
*/


fn main() {
    // Variable binding
    let x = 5;

    // Expressions
    x;
    x+1;
    15;

    // Blocks are expressions too, so they can be used as values in assignments
    // The last expression in the block will be assigned to the place expression as a local variable
    // However, if the last expression ends with a semicolon, the return value will be ()

    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x
    };

    let z = {
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
                