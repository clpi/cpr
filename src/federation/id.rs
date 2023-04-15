use std::ops::Deref;

use rand::{Rng, RngCore, distributions::Alphanumeric};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct FedId(pub String);

impl Default for FedId {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let rid: String =  (0..16).map(|_| (&mut rng).sample(Alphanumeric) as char)
            .collect::<String>();
        Self(rid)
    }
}

impl FedId {
    pub fn new() -> FedId {
        FedId::default()
    }
}
impl Deref<> for FedId {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ToString for FedId {
    fn to_string(&self) -> String {
        self.0.to_owned()
    }
}

impl Into<String> for FedId {
    fn into(self) -> String {
        self.0
    }
}

