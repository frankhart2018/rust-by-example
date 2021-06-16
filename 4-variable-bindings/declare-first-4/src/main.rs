
fn main() {
    // Declare a variable binding
    let a_binding;

    {
        let x = 2;

        a_binding = x + x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);

    // The compiler forbids use of uninitialized variables 
    // as this would lead to undefined behaviour
}
                