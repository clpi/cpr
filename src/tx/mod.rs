pub mod id;

use tokio::time::Duration;
use std::time::SystemTime;
use id::TxId;

/// 
#[derive(Clone, Debug)]
pub struct Transaction {
    pub id: TxId,
    pub send: String,
    pub recv: String,
    pub amt: usize,
    pub timestamp: SystemTime,
    pub sig: Option<String>,
}
impl Default for Transaction {
    fn default() -> Self {
        Self {
            id: TxId::new(),
            send: String::new(),
            recv: String::new(),
            amt: 0,
            timestamp: SystemTime::now(),
            sig: None,
        }
    }
}

impl Transaction {
    pub fn new(send: &str, recv: &str, amt: usize) -> Self {
        Self {
            id: TxId::new(),
            send: send.into(),
            recv: recv.into(),
            amt: amt.into(),
            timestamp: SystemTime::now(),
            sig: None,
        }
    }
}

