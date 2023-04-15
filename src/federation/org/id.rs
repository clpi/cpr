use std::ops::Deref;

use rand::{Rng, RngCore, distributions::Alphanumeric};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct OrgId(pub String);

impl Default for OrgId {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let rid: String =  (0..16).map(|_| (&mut rng).sample(Alphanumeric) as char)
            .collect::<String>();
        Self(rid)
    }
}

impl OrgId {
    pub fn new() -> OrgId {
        OrgId::default()
    }
}
impl Deref<> for OrgId {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ToString for OrgId {
    fn to_string(&self) -> String {
        self.0.to_owned()
    }
}

impl Into<String> for OrgId {
    fn into(self) -> String {
        self.0
    }
}
