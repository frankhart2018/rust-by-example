
fn main() {
    // When data is bound by the same name immutably, it also freezes
    // Frozen data can't be modified until the immutable binding goes out of scope

    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable _mutable_integer
        let _mutable_integer = _mutable_integer;

        // Can't change value of _mutable_integer until the local immutable one goes out of scope
        // _mutable_integer = 50;
    }

    _mutable_integer = 3;
}
                