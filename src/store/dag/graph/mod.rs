use petgraph::{
    Directed, Direction, EdgeDirection,
    algo::{astar, scc},
    csr::{Csr, Edges, NodeIdentifiers, Neighbors},
    adj::{List as AdjList, UnweightedList},
    graph::{Graph, GraphIndex, NodeIndex,}
};
use std::{
    hash::Hash, 
    collections::{BTreeMap, HashSet},
};
use crate::{Transaction, OrgUser, Org, Federation, Balance};
use tokio::time::Instant;


pub type TxGraph = Graph<Instant, Transaction, Directed, usize>;

#[derive(Debug, Clone)]
pub(crate) struct SparseDAG {
    pub federations: Vec<NodeIdentifiers>,
    pub users: Vec<NodeIdentifiers>,
}
