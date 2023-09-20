fn foo(x : i32, y : i32) -> i32 {
    x + y
}

struct C {
    x : i32
}

impl C {
    fn new(x : i32) -> Self {
        Self{x}
    }

    fn get_x(&self) -> i32 {
        self.x
    }
}

fn main() {
    println!("{:?}", foo(2, 3));

    let c = C::new(123);
    println!("{:?}", c.get_x());
}
