
fn main() {
    // This is an example of a line comment
    // There are two slashes at the beginning of the line
    // And nothing written inside these will be read by the compiler

    // println!("Hello, world!");

    /*
        * This is another type of comment, a block comment. In general,
        * line comments are the recommended comment style. But
        * block comments are extremely useful for temporarily disabling
        * chunks of code. /* Block comments can be  /* nested, */ */
    */

    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}
                