pub mod id;
pub mod user;

pub use self::id::OrgId;
use super::{FedId, Federation, HasIdentifier};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, ops::DerefMut};
use user::OrgUser;
use crate::models::Balance;

///
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde()]
pub struct Org {
    pub id: OrgId,
    #[serde(rename = "uppercase")]
    pub symbol: String,
    #[serde()]
    pub users: Vec<OrgUser>,
}

// impl Clone for Org {
//     fn clone(&self) -> Self {
//         let oc = Org::from(self);
//         Org { ..oc }
//     }
// }

impl Default for Org {
    fn default() -> Self {
        let oid = OrgId::gen();
        Self {
            id: oid.clone(),
            symbol: oid.to_string(),
            users: Vec::new(),
        }
    }
}

impl Org {
    pub fn new(name: &str) -> Self {
        Self {
            id: OrgId::new(name),
            users: Vec::new(),
            symbol: name.to_uppercase().into(),
        }
    }
    pub fn lookup(org_id: OrgId) -> Option<Self> {
        return Some(Self::default())
    }
    pub fn get_id(&self) -> String {
        return self.id.id.clone();
    }
    pub fn get_name(&self) -> String {
        return self.id.handle.clone();
    }
    pub fn get_identifier(&self) -> String {
        self.id.to_string()
    }
    pub fn new_user(&mut self, handle: String) -> OrgUser {
        let ou = OrgUser::new(self.id.clone(), handle.clone());
        self.users.push(OrgUser::new(self.id.clone(), handle.clone()));
        return ou;
    }
    pub fn get_users(self) -> Vec<OrgUser> {
        return Vec::from(self.users);
    }
    pub fn has_user(self, handle: String) -> Option<OrgUser> {
        let u = self
            .users
            .into_iter()
            .flat_map(|u| if u.id.handle == handle { Some(u) } else { None });
        let u = u.into_iter().find(|u| u.id.handle == handle);
        if u.is_none() {
            return None;
        } else {
            return Some(u.unwrap());
        }
    }
    pub fn get_or_create_user(&mut self, handle: String) -> OrgUser {
        let u = &mut self.users;
        match &u.into_iter().find(|u| u.id.handle == handle.clone()) {
            Some(u) => return OrgUser::new(self.id.clone(), handle.clone()),
            None => {
                let u = OrgUser::new(self.id.clone(), handle.clone());
                self.users.push(OrgUser::new(self.id.clone(), handle.clone()));
                return u;
            }
        }
    }
    pub fn new_from(name: &str, symbol: &str, users: Vec<OrgUser>) -> Self {
        Self {
            id: OrgId::new(name),
            users: Vec::new(),
            symbol: symbol.to_uppercase().into(),
        }
    }
}

impl fmt::Display for Org {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{};", self.id.handle, Self::get_discriminator(), self.id.id)
    }
}
