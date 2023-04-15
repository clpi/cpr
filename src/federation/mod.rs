pub mod org;
pub mod id;

pub use org::Org;
use self::{id::FedId, org::OrgId};

use super::Transaction;

use serde::{Serialize, Deserialize};
use tokio::time::Duration;
use std::{
    collections::{HashMap, VecDeque},
    time::SystemTime
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Federation {
    pub id: FedId,
    pub name: String,
    pub orgs: Vec<Org>
}

impl Default for Federation {
    fn default() -> Self {
        let fid = FedId::new();
        Self {
            id: fid.clone(),
            name: fid.to_string(),
            orgs: Vec::new(),

        }
    }
}

impl Federation {
    pub fn new(name: &str) -> Self {
        Self {
            id: FedId::new(),
            name: name.into(),
            orgs: Vec::<Org>::new(),
        }
    }

    pub fn register_org(&mut self, org: Org) {
        self.orgs.push(org);
    }

    pub fn validate_tx(&self, tx: &Transaction, org_id: OrgId) -> Option<String> {
        let orgs = self.orgs.as_slice();
        match orgs.into_iter().find(|o| o.id.name == org_id.name) {
            Some(o) => {
                // check validation
                let is_valid = true;
                if is_valid {
                    return Some(format!("{}: {:?}", o, tx));
                } else {
                    return None
                }
            },
            None => return None
        }
    }
}

