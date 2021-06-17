#![allow(overflowing_literals)]

fn main() {
    // Rust doesn't provide implicit type conversion (coercion)

    // Rules for explicit type conversion are same as C
    // except in cases where C has undefined behaviour

    let decimal = 65.4321_f32;

    // No implicit conversion
    // let integer: u8 = decimal;

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // A float cannot be directly converted to char
    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // When casting any value to an unsigned type T,
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type

    println!("1000 as u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 LSB are kept
    // while the rest towards MSB gets truncated
    println!("1000 as u8 is: {}", 1000 as u8);

    // -1 + 256 = 255
    println!("-1 as a u8 is: {}", (-1i8) as u8);

    // For positive numbers, this is same as modulus
    println!("1000 mod 256 is: {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type
    // If the MSB of that value is 1, then the value is negative

    // Unless it already fits
    println!("128 as a i16 is: {}", 128 as i16);

    // 128 as u8 -> 128
    println!("128 as a i8 is: {}", 128 as i8);

    println!("1000 as u8 is: {}", 1000 as u8);
    println!("232 as i8 is: {}", 232 as i8);

    println!("300.0 is {}", 300.0_f32 as u8);
    println!("-100.0 as u8 is {}", -100.0_f32 as u8);
    println!("nan as u8 is {}", f32::NAN as u8);

    // This behaviour incurs a small runtime cost and can be avoided with unsafe methods
    // however the results might overflow and return unsound values
    unsafe {
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
                