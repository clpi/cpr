pub mod tx;

use serde::{Serialize, Deserialize};
pub use tx::{TxId, Transaction};

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkMessage {
    Tx(Transaction),
    ValidationReq(Transaction),
    ValidationRes(Transaction, bool),
}
