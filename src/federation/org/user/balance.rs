use std::{
    sync::atomic::{Ordering, AtomicUsize},
    collections::HashMap,
};

pub type Symbol = String;
pub type Balances = Vec<Balance>;

#[derive(Debug, Default)]
pub struct Balance {
    // org_id: String,
    pub symbol: Symbol,
    pub amt: AtomicUsize,
}
impl Clone for Balance {
    fn clone(&self) -> Self {
        Self {
            symbol: self.symbol.clone(),
            amt: AtomicUsize::new(self.amt.load(Ordering::Relaxed)),
        }
    }
}

impl Balance {
    pub fn get(&self) -> usize {
        self.amt.load(Ordering::Relaxed)
    }
    pub fn add(&mut self, amt: usize) {
        self.amt.fetch_add(amt, Ordering::Relaxed);
    }
    pub fn sub(&mut self, amt: usize) {
        self.amt.fetch_sub(amt, Ordering::Relaxed);
    }
    pub fn zero() -> Self {
        Self::default()
    }

}
