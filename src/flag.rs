enum Something {
    On,
    Off,
}

fn foo(is_something: Something) {

    match is_something {
        Something::On => {
            println!("Something is ON");
        },
        _ => {
            println!("Something is OFF");
        },
    }
}

fn main() {
    foo(Something::On);
    foo(Something::Off);
}

