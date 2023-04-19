pub mod id;
pub mod key;

use serde::{Deserialize, Serialize};
use std::{ops::DerefMut, str::FromStr};

use crate::Balance;

use super::{HasIdentifier, OrgId};
pub use id::OrgUserId;

#[derive(Debug, PartialEq, Serialize, Clone, Deserialize)]
#[serde()]
pub struct OrgUser {
    pub id: OrgUserId,
    pub balances: Vec<Balance>,
}

impl Default for OrgUser {
    fn default() -> Self {
        Self {
            id: OrgUserId::default(),
            balances: vec![],
        }
    }
}

impl OrgUser {
    pub fn new(org_id: OrgId, handle: String) -> Self {
        Self {
            id: OrgUserId::new(org_id, handle),
            balances: vec![],
        }
    }
    pub fn get_global_identifier(self) -> String {
        format!("{} {} {}", 
            self.id.org_id.fed_id.to_string(),
            self.id.org_id.to_string(), 
            self.id.to_string())
    }

    pub fn get_in_fed_identifier(self) -> String {
        format!("{} {} {}", 
            self.id.org_id.fed_id.to_string(),
            self.id.org_id.to_string(), 
            self.id.to_string())
    }
    pub fn get_in_org_identifier(self) -> String {
        format!("{} {}", self.id.org_id.to_string(), self.id.to_string())
    }
    pub fn new_with_org_id(org_id: OrgId, handle: String) -> Self {
        Self {
            id: OrgUserId::new(org_id, handle),
            ..Default::default()
        }
    }
    pub fn new_with_identifier(org_identifier: String, handle: String) -> Self {
        let org_id = OrgId::from_str(&org_identifier.as_str()).unwrap_or_default();
        Self {
            id: OrgUserId::new(org_id, handle),
            balances: vec![],
        }
    }

    pub fn get_balance(&self, symbol: &str) -> Option<&Balance> {
        let b = self.balances.as_slice().into_iter().find_map(|b| {
            if b.symbol == symbol {
                Some(b)
            } else {
                None
            }
        });
        return b;
    }

    pub fn get_balance_mut(&mut self, symbol: &str) -> Option<&mut Balance> {
        let b = self.balances.deref_mut().into_iter().find_map(|b| {
            if b.symbol == symbol {
                Some(b)
            } else {
                None
            }
        });
        return b;
    }

    pub fn get_org_id(&self) -> OrgId {
        self.id.org_id.clone()
    }

    pub fn add_balance(&mut self, symbol: String, amt: usize) {
        if let Some(b) = self.get_balance_mut(&symbol) {
            b.add(amt);
        } else {
            self.balances.push(Balance::new(symbol, amt));
        }
    }

    pub fn sub_balance(&mut self, symbol: &str, amt: usize) {
        if let Some(b) = self.get_balance_mut(symbol) {
            b.sub(amt);
        }
    }
}
