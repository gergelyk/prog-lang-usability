use futures::future::join_all;
use rand::Rng;
use tokio::time::{Duration, sleep};

const N: i32 = 10;
const T: usize = 5;

async fn shift(x: i32) -> i32 {
    let random = {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..10)
    };
    sleep(Duration::from_millis(random)).await;
    println!("Done with {}", x);
    return 100 * x;
}

async fn launch(rt: &tokio::runtime::Runtime) {
    let tasks: Vec<_> = (0..N).map(|x| rt.spawn(shift(x))).collect();
    println!("All spawned");
    let results = join_all(tasks).await;
    println!("Results: {:?}", results.iter().collect::<Vec<_>>());
}

fn main() -> () {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(T)
        .enable_time()
        .build()
        .unwrap();
    rt.block_on(launch(&rt));
}
