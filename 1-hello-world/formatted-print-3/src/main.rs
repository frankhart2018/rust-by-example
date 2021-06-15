/*
 * format! -> Write formatted text to string.
 * print! -> Same as format! but the text is printed to the console (io::stdout).
 * println! -> Same as print! but a newline is appended.
 * eprint! -> Same as format! but the text is printed to the standard error (io::stderr).
 * eprintln! -> Same as eprint! but a newline is appended. 
*/


fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becomes i32. We can change the type of 31 by
    // providing a suffix. The number 31i64 has type i64.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps over");

    // We can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // We can pad numbers with extra zeros. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    println!("The struct `{:?}` won't print....", Structure(3));

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
}
                