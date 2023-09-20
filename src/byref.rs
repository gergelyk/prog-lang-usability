// in Rust there is no function overloading, we need to add suffixes to the functions
fn inc_i32(x: i32 ) -> i32 {
    x + 1
}

// in Rust we cannot provide implementatinon for a type (e.g. bool),
// but we can provide implementatinon for a trait (e.g. Not - those types that respond to operator `!`)
// I'm not sure how to implement it for std::ops::Add, because `1` must be of the same type as `x`
fn inc_inv<T>(x: impl std::ops::Not<Output = T> ) -> T {
    !x
}

fn inc_i32_inplace(x: &mut i32 ) {
    *x += 1
}

fn main() {
    println!("{}", inc_i32(3) );
    println!("{}", inc_inv(true) );
    
    let mut x = 8;
    inc_i32_inplace(&mut x);
    println!("{}", x);
}
