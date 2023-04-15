use tokio::{
    time::{Duration},
};
use crate::{Transaction, Federation};
use std::{
    thread,
    collections::VecDeque,
    sync::{Arc, Mutex,  atomic::{Ordering, AtomicUsize, AtomicBool}}, fmt,
};

#[derive(Debug, Default)]
pub struct DAG {
    pub nodes: Mutex<Vec<Transaction>>,
}
impl fmt::Display for DAG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", self.nodes.lock().unwrap().to_vec())
    }
}
impl DAG {
    pub fn new() -> Arc<DAG> {
        Arc::new(DAG { nodes: Mutex::new(Vec::new()) })
    }

    pub fn push_tx(&self, tx: Transaction) -> () {
        let mut nodes = self.nodes.lock().unwrap();
        nodes.push(tx);
    }
}

#[derive(Debug)]
pub struct StreamingDAG {
    pub dag: Arc<DAG>,
    pub window_size: AtomicUsize,
    pub tx_queue: Arc<Mutex<VecDeque<Transaction>>>,
    pub federation: Arc<Federation>,
}
impl fmt::Display for StreamingDAG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.dag)
    }
}
impl Default for StreamingDAG {
    fn default() -> Self {
        Self {
            federation: Arc::new(Federation::new("")),
            ..Default::default()
        }
    }
}

impl StreamingDAG {

    pub fn new_arc(window_size: usize) -> Arc<StreamingDAG> {
        Arc::new(Self::new(window_size))
    }

    pub fn new_arc_with_federation(window_size: usize, fed: Federation) -> Arc<Self> {
        Arc::new(Self::new_with_federation(window_size, fed))
    }

    pub fn new(window_size: usize) -> StreamingDAG {
        Self {
            dag: DAG::new(),
            window_size: AtomicUsize::new(window_size),
            tx_queue: Arc::new(Mutex::new(VecDeque::with_capacity(window_size))),
            federation: Arc::new(Federation::new(""))
        }
    }

    pub fn new_with_federation(window_size: usize, federation: Federation) -> Self {
        Self {
            dag: DAG::new(),
            federation: Arc::new(federation),
            window_size: AtomicUsize::new(window_size),
            tx_queue: Arc::new(Mutex::new(VecDeque::with_capacity(window_size))),
        }

    }

    pub async fn push_tx(&self, tx: Transaction, org: &str) {
        if let Some(sig) = self.federation.validate_tx(&tx, org) {
            let mut txn = tx.clone();
            txn.sig = Some(sig);
            self.dag.push_tx(txn.clone());
            let mut txnqueue = self.tx_queue.lock().unwrap();
            txnqueue.push_back(txn);
            if txnqueue.len() > self.window_size.load(Ordering::Relaxed) {
                txnqueue.pop_front();
            }
        }
    }

    pub async fn process_tx(&self, stop: &AtomicBool) {
        while !stop.load(Ordering::Relaxed) {
            let mut txnqueue = self.tx_queue.lock().unwrap();
            if let Some(txn) = txnqueue.pop_front() {
                println!("Processed transaction: {} -> {}, {}",
                         txn.send, txn.recv, txn.amt);
            }
            drop(txnqueue);
            thread::sleep(Duration::from_millis(100));
        }
    }
}
