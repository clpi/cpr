pub mod balance;
pub mod id;

use std::ops::DerefMut;

pub use id::Id;
pub use balance::{Balance, Balances};

#[derive(Debug, Clone)]
pub struct OrgUser {
    pub id: Id,
    pub handle: String,
    pub balances: Vec<Balance>,
}

impl Default for OrgUser {
    fn default() -> Self {
        let did = Id::new();
        Self {
            id: did.clone(),
            handle: did.to_string(),
            balances: vec![],
        }
    }
}

impl OrgUser {
    pub fn new(handle: String) -> Self {
        Self {
            id: Id::new(),
            handle,
            balances: vec![],
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
