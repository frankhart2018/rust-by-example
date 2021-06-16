enum WebEvent {
    // An enum may either be unit-like
    PageLoad,
    PageUnload,

    // Like tuple structs
    KeyPress(char),
    Paste(String),

    // Or C-like structures
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::KeyPress(c) => println!("Pressed '{}'", c),
        WebEvent::Paste(s) => println!("Pasted \"{}\"", s),
        WebEvent::Click { x, y } => {
            println!("Clicked at x={}, y={}", x, y);
        },
    }
}

#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Create a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

// The most common place where an alias is used is in impl block -> Self

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let x = Operations::Add;
    println!("{:?}", x);
}
                