pub mod federation;
pub mod models;
pub mod msg;
pub mod node;
pub mod store;

pub use federation::{Federation, Org};
pub use models::{Balance, Balances};
pub use msg::{Transaction, TxId};
pub use node::Node;
pub use store::{StreamingDAG, DAG};

use models::{HasIdentifier, Id};

use rand::Rng;
use std::{
    collections::{HashMap, VecDeque},
    fmt,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    thread,
    time::{Duration, SystemTime},
};
use tokio::sync::Mutex;

use crate::federation::org::user::OrgUser;

pub async fn run() {
    println!("RUNNING");
    let mut fed = Federation::new("test");
    let o1 = Org::new("Alice");
    let o2 = Org::new("Bob");
    fed.register_org(o1.clone());
    fed.register_org(o2.clone());
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
        let us1 = OrgUser::new(o1.clone().id, u1);
        let us2 = OrgUser::new(o2.clone().id, u2);

        let amt = rng.gen_range(1..=100);
        let (mut recv, mut send) = if rng.gen_bool(0.5) {
            (us2, us1)
        } else {
            (us1, us2)
        };
        let (org, symbol) = if rng.gen_bool(0.5) {
            (o1.clone().id, o1.clone().symbol)
        } else {
            (o2.clone().id, o2.clone().symbol)
        };

        let tx = Transaction::new(send.clone(), recv.clone(), &symbol, amt);
        println!(
            "IN \x1b[32;1m{}\x1b[0m: \x1b[33;1m{}\x1b[0m PAID \x1b[34;1m{}\x1b[0m \x1b[35;1m{}{}\x1b[0m",
            // org.clone().handle,
            org.clone().get_global_identifier(),
            // sendid.clone().org_id.to_string(),
            // sendid.clone().handle,
            &send.clone().get_global_identifier(),
            &recv.clone().get_global_identifier(),
            // recvid.clone().org_id.to_string(),
            // recvid.clone().handle,
            // recvid.clone().to_string(),
            amt.clone(),
            symbol.clone(),
        );
        recv.add_balance(symbol.clone(), amt.clone());
        streamdag.push_tx(tx, org).await;

        println!(
            "DAG [{}]: {:#?}",
            streamdag.dag.nodes.lock().unwrap().len(),
            streamdag
        );
        thread::sleep(Duration::from_millis(6600));
    }
}
