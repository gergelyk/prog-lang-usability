use std::time;
use std::thread;
use rand::Rng;

const N: i32 = 100;
const T: i32 = 3;

fn shift(q_in: flume::Receiver<Option<i32>>, q_out: flume::Sender<Option<i32>>, offset: i32) {
    let ms = time::Duration::from_millis(1);
    let mut rng = rand::thread_rng();
    loop {
        let x = q_in.recv().unwrap();
        match x {
            Some(v) => { q_out.send( Some(v + offset) ).unwrap(); }
            None => { break; }
        }
        thread::sleep(rng.gen_range(0..10) * ms);
    }
}

fn main() {
    let (q_in_tx, q_in_rx) = flume::unbounded();
    let (q_out_tx, q_out_rx) = flume::unbounded();

    let mut t = Vec::<thread::JoinHandle<()>>::new();

    for x in 0..T {
        let q_in_rx_c = q_in_rx.clone();
        let q_out_tx_c = q_out_tx.clone();
        t.push(thread::spawn(move || {
            shift(q_in_rx_c, q_out_tx_c, (x + 1) * 100);
        }));
     }
     
    for x in 0..N {
        q_in_tx.send(Some(x)).unwrap();
    }

    for _ in 0..T {
        q_in_tx.send(None).unwrap();
    }

    for _ in 0..T {
        t.pop().unwrap().join().unwrap();
    }
    
    for _ in 0..N {
        if let Some(res) = q_out_rx.recv().unwrap() {
            println!("{:?}", res);
        }
    }
    
}
