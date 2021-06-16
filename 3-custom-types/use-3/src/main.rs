#[allow(unused_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use crate::Status::{Poor, Rich};

    use crate::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money"),
        Poor => println!("The poor have no money..."),
    };

    match work {
        Civilian => println!("Civilians work!"),
        Solider => println!("Soldiers fight!"),
    };
}
                