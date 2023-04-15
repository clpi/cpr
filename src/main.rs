pub mod node;
pub mod federation;
pub mod tx;
pub mod store;

pub use tx::Transaction;
pub use federation::{Federation, Org};
pub use store::{DAG, StreamingDAG};

use rand::Rng;
use tokio::{
    sync::Mutex
};
use std::{
    fmt, thread,
    collections::{HashMap, VecDeque},
    sync::{Arc, 
        atomic::{AtomicBool, Ordering, AtomicUsize},
    },
    time::{Duration, SystemTime},
};


pub async fn run() {
    println!("RUNNING");
    let mut fed = Federation::new("test");
    fed.register_org(Org::new("Alice"));
    fed.register_org(Org::new("Bob"));
    let streamdag = StreamingDAG::new_arc_with_federation(10, fed);
    // streamdag.federation = fed;
    let stop = Arc::new(AtomicBool::new(false));
    // let _proc_thread = thread::spawn(move || {
    //     sdag_clone.process_tx(&stop_clone);
    // });
    let mut rng = rand::thread_rng();
    loop {
        println!("LOOPING");
        let sender = format!("User{}", rng.gen_range(1..=10));
        let recv = format!("User{}", rng.gen_range(1..=10));
        let amt = rng.gen_range(1..=100);

        let tx = Transaction::new(&sender, &recv, amt);
        let org_name = if rng.gen_bool(0.5) { "Alice" } else { "Bob" };
        println!("IN {}, {} PAID {} {}{}",
                 org_name,
                 sender,
                 recv,
                 amt,
                 org_name);
        streamdag.push_tx(tx, org_name).await;

        println!("DAG [{}]: {:#?}", streamdag.dag.nodes.lock().unwrap().len(), streamdag);
        thread::sleep(Duration::from_millis(2600));
    }
}

#[tokio::main]
async fn main() {
    run().await;
}
