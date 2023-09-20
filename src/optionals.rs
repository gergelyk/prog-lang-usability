fn foo(x: Option<i32>) {
    
    let x = match x {
        None => 123,
        Some(x) => x,
    };
    
    println!("x={}", x);
}

fn main() {
    foo(Some(321)); // x=321
    foo(Some(0));   // x=0
    foo(None);      // x=123
}
