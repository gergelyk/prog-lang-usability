#[macro_use(c)]
extern crate cute;

fn main() {
    let data: Vec<i32> = vec![2, 3, 7, 4, 1];
    let sqr = c![x.pow(2), for x in data];
    let lim = c![x, for x in sqr, if x < 20];
    let res: i32 = lim.iter().sum();
    dbg!(res);
}
