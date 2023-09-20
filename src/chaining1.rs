fn main() {
    let data = [2, 3, 7, 4, 1];
    let res = data.iter()
                .map(|x: &i32| x.pow(2))
                .filter(|x| x < &20)
                .sum::<i32>();
                
    println!("{:?}", res);
}
