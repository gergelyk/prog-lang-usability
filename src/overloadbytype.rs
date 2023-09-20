#![feature(auto_traits)]
#![feature(negative_impls)]
use std::ops::Neg;
use std::any::type_name;

auto trait Flippable {
}

impl !Flippable for bool {}
impl !Flippable for i32 {}
impl !Flippable for i64 {}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

fn flip_num<T: Neg<Output = T>> (a: T) -> T {
    -a
}

trait Flip {
    fn flip(a: Self) -> Self;
}

impl Flip for bool {
    fn flip(a: bool) -> bool {
        !a
    }
}

impl Flip for i32 {
    fn flip(a: i32) -> i32 {
        flip_num(a)
    }
}

impl Flip for i64 {
    fn flip(a: i64) -> i64 {
        flip_num(a)
    }
}

impl Flip for Point {
    fn flip(a: Point) -> Point {
        Point{x: a.y, y: a.x}
    }
}

impl<T> Flip for T
where
    T: Flippable
{
    fn flip(a: T) -> T {
        println!("Warning: Type {} not flippable", type_name::<T>() );
        a
    }
}

fn flip<T: Flip>(a: T) -> T {
    Flip::flip(a)
}

fn main() {
    // types known in advance
    dbg!( flip(true));
    dbg!( flip(3 as i32) );
    dbg!( flip(33 as i64) );
    dbg!( flip(Point{x: 5, y: 7}) );
    dbg!( flip(33.3) );
    
    // types determined in runtime
    /* Error: not possible to use it this way
    let items: Vec<&dyn std::any::Any> = vec![&true, &(3 as i32), &(3 as i64), &Point{x: 5, y: 7}, &33.3];
    for item in items {
        flip(*item);
    }
    */
}
