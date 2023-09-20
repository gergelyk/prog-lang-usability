#[derive(Default)]
struct Foo {
    bar: bool,
    baz: i32,
    qux: String,
}

impl Foo {
    fn default() -> Self {
        Self{
            baz: 999,
            ..Default::default()
        }
    }

    fn bar(mut self, val: bool) -> Self {
        self.bar = val;
        self
    }
    
    fn baz(mut self, val: i32) -> Self {
        self.baz = val;
        self
    }
    
    fn qux(mut self, val: String) -> Self {
        self.qux = val;
        self
    }
    
    fn call(&self) {
        println!("bar={}, baz={}, qux={}", self.bar, self.baz, self.qux);
    }
}

fn main() {
    Foo{bar: true, baz: 123, qux: "abc".to_string()}.call();
    Foo{bar: true, ..Default::default()}.call();
    Foo::default().call();
    Foo::default().qux("hello".to_string()).call();
    Foo::default().qux("hello".to_string()).bar(true).baz(987).call();
}
