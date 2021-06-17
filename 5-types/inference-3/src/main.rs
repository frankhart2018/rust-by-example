
fn main() {
    let elem = 5u8;

    let mut vec = Vec::new();

    // Compiler infers the type from this push
    vec.push(elem);

    println!("{:?}", vec);
}
                