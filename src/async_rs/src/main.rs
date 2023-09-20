use std::time::Duration;
use async_std::task;
use futures::executor::block_on;
use futures::future::join_all;
use rand::Rng;

const N: i32 = 10;

async fn shift(x: i32) -> i32 {
    let ms = Duration::from_millis(1);
    let random = {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..10)
    };

    task::sleep(random * ms).await;
    println!("Done with {}", x);
    return 100 * x;
}

async fn launch() {
    let tasks: Vec<_> = (0..N).map(|x| task::spawn(shift(x))).collect();
    println!("All spawned");
    let results = join_all(tasks).await;
    println!("Results: {:?}", results.iter().collect::<Vec<_>>());
}

fn main() {
    block_on(launch());
}
