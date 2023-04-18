pub mod id;
pub mod org;

use self::{id::FedId, org::OrgId};
use super::Transaction;
pub use super::models::HasIdentifier;
pub use org::Org;

use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, VecDeque},
    time::SystemTime,
};
use tokio::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
#[serde()]
pub struct Federation {
    pub id: FedId,
    pub orgs: Vec<Org>,
}

impl Clone for Federation {
    fn clone(&self) -> Self {
        Federation { ..self.clone() }
    }
}

impl Default for Federation {
    fn default() -> Self {
        let fid = FedId::new("".into());
        Self {
            id: fid.clone(),
            orgs: Vec::new(),
        }
    }
}

impl Federation {
    pub fn new(handle: &str) -> Self {
        Self {
            id: FedId::new(handle.into()),
            orgs: Vec::<Org>::new(),
        }
    }

    pub fn register_org(&mut self, org: Org) {
        self.orgs.push(org);
    }

    pub fn validate_tx(&self, tx: &Transaction, org_id: OrgId) -> Option<String> {
        let orgs = self.orgs.as_slice();
        match orgs.into_iter().find(|o| o.id.handle == org_id.handle) {
            Some(o) => {
                // check validation
                let is_valid = true;
                if is_valid {
                    return Some(format!("{}: {:?}", o, tx));
                } else {
                    return None;
                }
            }
            None => return None,
        }
    }
}
