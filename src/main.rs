pub mod node;
pub mod federation;
pub mod tx;
pub mod store;

pub use node::Node;
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

use crate::federation::org::user::OrgUser;


pub async fn run() {
    println!("RUNNING");
    let mut fed = Federation::new("test");
    let o1 = Org::new("Alice");
    let o2 = Org::new("Bob");
    fed.register_org(o1);
    fed.register_org(o2);
    let streamdag = StreamingDAG::new_arc_with_federation(10, fed);
    // streamdag.federation = fed;
    let stop = Arc::new(AtomicBool::new(false));
    // let _proc_thread = thread::spawn(move || {
    //     sdag_clone.process_tx(&stop_clone);
    // });
    let mut rng = rand::thread_rng();
    loop {
        println!("LOOPING");
        let u1 = format!("User{}", rng.gen_range(1..=10));
        let u2 = format!("User{}", rng.gen_range(1..=10));
        let us1 = OrgUser::new(o1.id, u1);
        let us2 = OrgUser::new(o2.id, u2);

        let amt = rng.gen_range(1..=100);
        let (recv, send) = if rng.gen_bool(0.5) { 
            (us2, us1)
        } else { (us1, us2)};
        let (org, symbol) = if rng.gen_bool(0.5) { 
            (o1.id, o1.symbol)
        } else { 
            (o2.id, o2.symbol)
        };
        

        let tx = Transaction::new(send, recv, &symbol, amt);
        println!("IN {} ({}), {} {}{} PAID {} {}{} {}{}",
                 org.name,
                 org.to_string(),
                 send.org_id.to_string(), send.handle, send.id.to_string(),
                 recv.org_id.to_string(), recv.handle, recv.id.to_string(),
                 amt, symbol,
                 );
        streamdag.push_tx(tx, org).await;

        println!("DAG [{}]: {:#?}", streamdag.dag.nodes.lock().unwrap().len(), streamdag);
        thread::sleep(Duration::from_millis(2600));
    }
}

#[tokio::main]
async fn main() {
    run().await;
}
