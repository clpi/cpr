use std::ops::Deref;
use serde::{Serialize, Deserialize};

use rand::{Rng, RngCore, distributions::Alphanumeric};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Id(pub String);

impl Default for Id {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let rid: String =  (0..16).map(|_| (&mut rng).sample(Alphanumeric) as char)
            .collect::<String>();
        Self(rid)
    }
}

impl Id {
    pub fn new() -> Id {
        Id::default()
    }
}
impl Deref<> for Id {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ToString for Id {
    fn to_string(&self) -> String {
        self.0.to_owned()
    }
}

impl Into<String> for Id {
    fn into(self) -> String {
        self.0
    }
}
