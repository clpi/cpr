pub mod balance;
pub mod id;

use serde::{Serialize, Deserialize};
use std::ops::DerefMut;

pub use id::Id;
pub use balance::{Balance, Balances};

use super::OrgId;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrgUser {
    pub id: Id,
    pub org_id: OrgId,
    pub handle: String,
    pub balances: Vec<Balance>,
}

impl Default for OrgUser {
    fn default() -> Self {
        let did = Id::new();
        Self {
            id: did.clone(),
            org_id: OrgId::gen(),
            handle: did.to_string(),
            balances: vec![],
        }
    }
}

impl OrgUser {
    pub fn new(org_id: OrgId, handle: String) -> Self {
        Self {
            id: Id::new(),
            org_id,
            handle,
            balances: vec![],
        }
    }
    pub fn get_global_identifier(self) -> String {

    }
    pub fn get_in_fed_identifier(self) -> String {
        format!("{}{}", self.org_id.to_self.get_in_org_identifier())

    }
    pub fn get_in_org_identifier(self) -> String {
        format!("{}{}", self.handle, self.id)
    }
    pub fn new_with_org_id(org_id: OrgId, handle: String) -> Self {
        Self {
            id: Id::new(),
            org_id, handle, ..Default::default()
        }

    }
    pub fn new_with_identifier(org_identifier: String, handle: String) -> Self {
        Self {
            id: Id::new(),
            org_id: OrgId::try_from(org_identifier).unwrap_or_default(),
            balances: vec![],
            handle,
        }

    }

    pub fn get_balance_mut(&mut self, symbol: &str) -> Option<&mut Balance> {
        let b = self.balances.deref_mut().into_iter().find_map(|b| {
            if b.symbol == symbol {
                Some(b)
            } else {
                None
            }
        });
        if b.is_none() {
            return None;
        } else {
            return Some(b.unwrap());
        }
    }

    pub fn get_org_id(&self) -> OrgId {
        self.org_id.clone()
    }

    pub fn add_balance(&mut self, symbol: &str, amt: usize) {
        if let Some(b) = self.get_balance_mut(symbol) {
            b.add(amt);
        }
    }

    pub fn sub_balance(&mut self, symbol: &str, amt: usize) {
        if let Some(b) = self.get_balance_mut(symbol) {
            b.sub(amt);
        }
    }

}
