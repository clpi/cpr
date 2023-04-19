use crate::Transaction;

#[derive(Clone, Debug, PartialEq)]
pub struct DAGNode {
    tx: Transaction,
    children: Box<Vec<DAGNode>>,
}

impl DAGNode {
    pub fn new(tx: Transaction) -> Self {
        Self {
            tx,
            children: Box::new(Vec::new()),
        }
    }
}
