struct Hundred {
}

impl std::ops::Add<Hundred> for i32 {
    type Output = i32;

    fn add(self, rhs: Hundred) -> i32 {
        rhs + self
    }
}

impl std::ops::Add<i32> for Hundred {
    type Output = i32;

    fn add(self, rhs: i32) -> i32 {
        rhs + 100
    }
}

fn main() {

    dbg!(2 + Hundred{}); // 102
    dbg!(Hundred{} + 4); // 104

    // Other numeric types require separate implementation
    //dbg!(Hundred{} + 4.5 );
}
