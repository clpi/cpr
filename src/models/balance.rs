use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

pub type Symbol = String;
pub type Balances = Vec<Balance>;

#[derive(Serialize, Deserialize, Debug, Default)]
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
    pub fn new(symbol: String, amt: usize) -> Self {
        Self {
            symbol,
            amt: AtomicUsize::new(amt),
        }
    }
    pub fn get(&self) -> usize {
        self.amt.load(Ordering::Relaxed)
    }
    pub fn add(&mut self, amt: usize) {
        self.amt.load(Ordering::Relaxed).wrapping_add(amt);
    }
    pub fn sub(&mut self, amt: usize) {
        self.amt.fetch_sub(amt, Ordering::Relaxed);
    }
    pub fn zero() -> Self {
        Self::default()
    }
}
