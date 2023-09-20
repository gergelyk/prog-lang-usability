fn main() {
    let data: Vec<i32> = vec![2, 3, 7, 4, 1];
    let res: i32 = data.iter()
                .map(|x| x.pow(2))
                .filter(|x| x < &20)
                .sum();
                
    println!("{:?}", res);
}
