use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma
            // Use the ? operator to return on errors
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    // Implementing fmt::Display for a structure where the elements
    // must each be handled sequentially is tricky
    // The problem is that each write! generates a fmt::Result
    // Proper handling of this requires dealing with all the results, we 
    // can use ? operator here

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
                