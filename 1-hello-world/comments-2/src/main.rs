
fn main() {
    // This is an example of a line comment
    // There are two slashes at the beginning of the line
    // And nothing written inside these will be read by the compiler

    // println!("Hello, world!");

    /*
     * This is another type of comment, a block comment. In general,
     * line comments are the recommended comment style. But
     * block comments are extremely useful for temporarily disabling 
     * chunks of code. /* Block comments can be /* nested,  */ */
     * so it takes only a few keystrokes to comment out everything
     * in this main() function. 
    */

    /*
    Note: The previous column of `*` was entirely for style. There's 
    no actual need for it.
    */

    // We can manipulate expressions more easily with block comments
    // than with line comments. Try deleting the comment delimiters
    // to change the results:
    let x = 5 + /* 90 + */ 5;
    println!("Its `x` or 100? x = {}", x);
}
                