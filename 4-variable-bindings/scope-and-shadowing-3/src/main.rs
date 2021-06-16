
fn main() {
    // This binding lives in the main function
    let long_lived_binding = 1;

    // This is a block and has a smaller scope than main function
    {
        // This binding exists only in this block
        let short_lived_binding = 2;

        println!("Inner short: {}", short_lived_binding);
    }

    // println!("Outer short: {}", short_lived_binding);

    println!("Outer long: {}", long_lived_binding);

    let shadowing_binding = 1;

    {
        println!("Before being shadowed: {}", shadowing_binding);

        let shadowing_binding = "abc";

        println!("Shadowed in the inner block: {}", shadowing_binding);
    }

    println!("Outside inner block: {}", shadowing_binding);

    let shadowing_binding = 2;
    println!("Shadowed in outer block: {}", shadowing_binding);
}
                