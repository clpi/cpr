pub mod user;
pub mod id;

use std::{fmt, collections::HashMap, ops::DerefMut};
use serde::{Serialize, Deserialize};
use user::{OrgUser, Balance};
pub use self::id::OrgId;


#[derive(Serialize, Deserialize, Debug)]
pub struct Org {
    pub id: OrgId,
    pub symbol: String,
    pub users: Vec<OrgUser>,
}

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
            symbol: name.to_uppercase().into() 
        }
    }
    pub fn get_id(&self) -> String {
        return self.id.id.clone();
    }
    pub fn get_name(&self) -> String {
        return self.id.name.clone();
    }
    pub fn get_identifier(&self) -> String {
        self.id.to_string()
    }
    pub fn new_user(&mut self, handle: String) -> OrgUser {
        let ou = OrgUser::new(self.id, handle.clone());
        self.users.push(OrgUser::new(self.id, handle.clone()));
        return ou;
    }
    pub fn get_users(self) -> Vec<OrgUser> {
        return Vec::from(self.users);
    }
    pub fn has_user(self, handle: String) -> Option<OrgUser> {
        let u = self.users.into_iter().flat_map(|u| {
            if u.handle == handle {
                Some(u)
            } else {
                None
            }
        });
        let u = u.into_iter().find(|u| u.handle == handle);
        if u.is_none() {
            return None;
        } else {
            return Some(u.unwrap());
        }
    }
    pub fn get_or_create_user(&mut self, handle: String) -> OrgUser {
        let u = &mut self.users;
        match &u.into_iter().find(|u| u.handle == handle.clone()) {
            Some(u) => return OrgUser::new(self.id, handle.clone()),
            None => {
                let u = OrgUser::new(self.id, handle.clone());
                self.users.push(OrgUser::new(self.id, handle));
                return u;
            },
        }
    }
    pub fn new_from(
        name: &str, 
        symbol: &str,
        users: Vec<OrgUser>,
    ) -> Self {
        Self { 
            id: OrgId::new(name),
            users: Vec::new(),
            symbol: symbol.to_uppercase().into(), 
        }
    }
}

impl fmt::Display for Org {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.id.id, self.id.name)
    }
}


