pub mod id;

use tokio::time::Duration;
use std::time::SystemTime;
pub use id::TxId;

use crate::{federation::org::user::OrgUser, Balance};


/// 
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    pub id: TxId,
    pub send: OrgUser,
    pub recv: OrgUser,
    pub amt: Balance,
    pub timestamp: SystemTime,
    pub sig: Option<String>,
    pub contract: Option<Vec<u8>>,
}
impl Clone for Transaction {
    fn clone(&self) -> Self {
        return Self {
            id: self.id.clone(),
            send: OrgUser::from(self.send.clone()),
            recv: OrgUser::from(self.recv.clone()),
            amt: self.amt.clone(),
            timestamp: self.timestamp,
            sig: self.sig.clone(),
            contract: self.contract.clone(),
        }
    }
}
impl Default for Transaction {
    fn default() -> Self {
        Self {
            id: TxId::new(),
            amt: Balance::new("".into(), 0),
            timestamp: SystemTime::now(),
            sig: None, ..Default::default()
        }
    }
}

impl Transaction {
    pub fn new(send: OrgUser, recv: OrgUser, symbol: &str, amt: usize) -> Self {
        Self {
            id: TxId::new(),
            send,
            recv,
            amt: Balance::new(symbol.into(), amt),
            timestamp: SystemTime::now(),
            sig: None,
            contract: None,
        }
    }
}

