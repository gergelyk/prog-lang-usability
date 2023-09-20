trait Dots {
    fn dots(&self) -> String;
}

impl Dots for i32 {
    fn dots(&self) -> String {
        ".".repeat(*self as usize)
    }
}

fn main() {
    // There are no properties, we need parentheses.
    println!("{}", 9.dots());
}
