pub mod tree;
pub mod graph;
pub mod node;

use petgraph::{
    graph::{Graph, NodeIndex},
    visit::{EdgeRef, IntoNodeReferences, NodeRef, Reversed},
    Direction,
};
use tokio::{
    time::{Duration},
};
use crate::{Transaction, Federation, federation::org::OrgId};
use std::{
    thread,
    collections::{VecDeque, HashMap, BTreeMap},
    sync::{Arc, Mutex,  atomic::{Ordering, AtomicUsize, AtomicBool}}, fmt,
};
use serde::{Serialize, Deserialize};

pub static MAX_BLOCK_SIZE_BYTES: usize = 1000000;
pub static MAX_BLOCK_SIZE_TXS: usize = 1000;
pub static BLOCK_RESPONSE_PREFIX_SIZE: usize = 4;
pub static BLOCK_RESPONSE_FIELD_KEY_SIZE: usize = 1;
pub static MAX_MIN_MSG_SIZE: usize = MAX_BLOCK_SIZE_BYTES
    + BLOCK_RESPONSE_PREFIX_SIZE
    + BLOCK_RESPONSE_FIELD_KEY_SIZE;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DAG {
    pub nodes: Mutex<Vec<Transaction>>,
    pub graph: Arc<Mutex<Graph<Transaction, ()>>>,
    pub tx_indices: Arc<Mutex<BTreeMap<Transaction, NodeIndex>>>,
}
impl fmt::Display for DAG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", self.nodes.lock().unwrap())
    }
}
impl DAG {
    pub fn new() -> Arc<DAG> {
        Arc::new(DAG { 
            nodes: Mutex::new(Vec::new()),
            graph: Arc::new(Mutex::new(Graph::<Transaction, ()>::new())),
            tx_indices: Arc::new(Mutex::new(BTreeMap::new())),
        })
    }

    pub async fn push_tx(&self, tx: Transaction, parent_txs: Vec<Transaction>) -> () {
        let mut graph: Graph<Transaction, ()> = self.graph.lock().await;
        let mut tx_indices: BTreeMap<Transaction, NodeIndex> = self.tx_indices.lock().await;
        let tx_nde = graph.add_node(tx.clone());
        tx_indices.insert(tx.clone(), tx_nde);
        for parent in parent_txs {
            if let Some(parent_node) = tx_indices.get(&parent) {
                graph.add_edge(*parent_node, tx_nde, ());
            }
        }
        // let mut nodes = self.nodes.lock().unwrap();
        // nodes.push(tx);
    }
    pub async fn get_parents(&self, tx: &Transaction) -> Vec<Transaction> {
        let g: Graph<Transaction, ()> = self.graph.lock().await;
        let tx_ind: BTreeMap<Transaction, NodeIndex> = self.tx_indices.lock().await;
        if let Some(txn) = tx_ind.get(tx) {
            g.neighbors_directed(*txn, Direction::Incoming)
                .map(|pnode| g[pnode].clone())
                .collect();
        } else {
            Vec::new()
        }
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

    pub async fn confirm_tx(&self, tx: &Transaction) -> () {
        let parents = self.dag.get_parents(tx).await;
        self.dag.push_tx(tx.clone(), parents).await;
    }
    // pub fn find_tx(&self, tx: &Transaction) -> bool {
    //     let mut nodes = self.dag.nodes.lock().unwrap();
    //     let mut found = false;
    //     for n in nodes.iter() {
    //         if n == tx {
    //             found = true;
    //             break;
    //         }
    //     }
    //     found
    // }

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

    pub async fn push_tx(&self, tx: Transaction, org_id: OrgId, parent_txs: Vec<Transaction>) {
        let mut queue: VecDeque<Transaction> = self.tx_queue.lock().await;
        queue.push_back(tx.clone());
        drop(queue);
        self.federation.validate_tx_distributed(&tx.clone()).await;
        self.dag.push_tx(tx, parent_txs).await;

        if let Some(sig) = self.federation.validate_tx(&tx, org_id) {
            let mut txn = Transaction::from(tx.clone());
            txn.sig = Some(sig);
            self.dag.push_tx(tx.clone(), parent_txs).await;
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
                println!("Processed transaction: {}{} -> {}{}, {}{}",
                         txn.send.get_org_id().to_string(), txn.send.id.handle,
                         txn.recv.get_org_id().to_string(), txn.recv.id.handle,
                         txn.amt.amt.load(Ordering::Relaxed), txn.amt.symbol);
            }
            drop(txnqueue);
            tokio::time::sleep(Duration::from_millis(1000)).await;
        }
    }
}
