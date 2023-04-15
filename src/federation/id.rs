use std::ops::Deref;

use rand::{Rng, RngCore, distributions::Alphanumeric};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct FedId {
    pub id: String,
    pub handle: String,
}

impl Default for FedId {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let rid: String =  (0..16).map(|_| (&mut rng).sample(Alphanumeric) as char)
            .collect::<String>();
        Self{ id: rid, handle: "".to_string(), }
    }
}

impl FedId {

    pub fn gen_id() -> String {
        let mut rng = rand::thread_rng();
        let rid: String =  (0..1).map(|_| (&mut rng).sample(Alphanumeric) as char)
            .collect::<String>();
        return rid
    }
    pub fn new(handle: String) -> FedId {
        Self {
            id: Self::gen_id(),
            handle,
        }
    }
    pub fn none() -> FedId {
        FedId::new("".to_string())
    }
    pub fn get_id(&self) -> String {
        self.id
    }
    pub fn get_name(&self) -> String {
        self.handle
    }
    pub fn get_identifier() -> String { 
        format!("{}{}", self.handle, self.id)
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

