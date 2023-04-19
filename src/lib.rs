pub mod federation;
pub mod models;
pub mod msg;
pub mod node;
pub mod store;
pub mod validate;

pub use validate::{Validator};
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

use crate::federation::{org::user::OrgUser, id::FedId};

pub async fn run() {
    println!("RUNNING");
    let mut fed = Federation::new("test");
    let fid = fed.id.clone();
    let mut o1: Org = Org::new_from(fid.clone(), "aliceorg", "alice", Vec::new());
    let mut o2: Org = Org::new_from(fid.clone(), "boborg", "bob", Vec::new());
    let mut o3: Org = Org::new_from(fid.clone(), "jimorg", "jim", Vec::new());
    let mut o4: Org = Org::new_from(fid.clone(), "lucyorg", "lucy", Vec::new());
    fed.register_orgs(&[o1.clone(), o2.clone(), o3.clone(), o4.clone()]);
    let streamdag = StreamingDAG::new_arc_with_federation(10, fed);
    // streamdag.federation = fed;
    let stop = Arc::new(AtomicBool::new(false));
    // let _proc_thread = thread::spawn(move || {
    //     sdag_clone.process_tx(&stop_clone);
    // });
    let mut rng = rand::thread_rng();
    loop {
        let u1 = format!("Jordan{}", rng.gen_range(1..=10));
        let u2 = format!("Tom{}", rng.gen_range(1..=10));
        let u3 = format!("Lester{}", rng.gen_range(1..=10));
        let u4 = format!("Irina{}", rng.gen_range(1..=10));
        let us1 = OrgUser::new(o1.clone().id, u1);
        let us2 = OrgUser::new(o2.clone().id, u2);
        let us3 = OrgUser::new(o3.clone().id, u3);
        let us4 = OrgUser::new(o4.clone().id, u4);

        o1.clone().new_user(us1.clone().id.handle);
        o2.clone().new_user(us2.clone().id.handle);
        o3.clone().new_user(us3.clone().id.handle);
        o4.clone().new_user(us4.clone().id.handle);

        let amt = rng.gen_range(1..=100);
        let (mut recv, mut send) = if rng.gen_bool(0.5) {
            if rng.gen_bool(0.5) {
                (us1.clone(), us2.clone())
            } else {
                (us2.clone(), us4.clone())
            }
        } else {
            if rng.gen_bool(0.5) {
                (us3.clone(), us1.clone())
            } else {
                (us4.clone(), us3.clone())
            }
        };

        let (org, symbol) = if rng.gen_bool(0.5) {
            if rng.gen_bool(0.5) {
                (o1.clone().id, o1.clone().symbol)
            } else {
                (o2.clone().id, o2.clone().symbol)
            }
        } else {
            if rng.gen_bool(0.5) {
                (o3.clone().id, o3.clone().symbol)
            } else {
                (o4.clone().id, o4.clone().symbol)
            }
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
        println!("TX QUEUE LEN: {} DAG LEN: {}", &streamdag.clone().tx_queue.lock().unwrap().len(), &streamdag.clone().dag.nodes.lock().unwrap().len() );
        recv.add_balance(symbol.clone(), amt.clone());
        streamdag.push_tx(tx, org).await;

        // println!(
        //     "DAG [{}]: {:#?}",
        //     streamdag.dag.nodes.lock().unwrap().len(),
        //     streamdag
        // );
        thread::sleep(Duration::from_millis(6600));
    }
}
