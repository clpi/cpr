use std::ops::Deref;

use rand::{Rng, RngCore, distributions::Alphanumeric};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct TxId(pub String);

impl Default for TxId {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let rid: String =  (0..16).map(|_| (&mut rng).sample(Alphanumeric) as char)
            .collect::<String>();
        Self(rid)
    }
}

impl TxId {
    pub fn new() -> TxId {
        TxId::default()
    }
}
impl Deref<> for TxId {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ToString for TxId {
    fn to_string(&self) -> String {
        self.0.to_owned()
    }
}

impl Into<String> for TxId {
    fn into(self) -> String {
        self.0
    }
}


