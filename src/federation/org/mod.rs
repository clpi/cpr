pub mod user;
pub mod id;

use std::{fmt, collections::HashMap, ops::DerefMut};
use user::{OrgUser, Balance};
pub use self::id::OrgId;


#[derive(Debug)]
pub struct Org {
    pub id: OrgId,
    pub name: String,
    pub identifier: String,
    pub symbol: String,
    pub users: Vec<OrgUser>,
}

impl Default for Org {
    fn default() -> Self {
        let oid = OrgId::new();
        Self {
            id: oid.clone(),
            name: oid.to_string(),
            identifier: oid.to_string(),
            symbol: oid.to_string(),
            users: Vec::new(),
        }
    }
}

impl Org {
    pub fn new(name: &str) -> Self {
        Self { 
            id: OrgId::new(),
            name: name.into(), 
            identifier: name.into(), 
            users: Vec::new(),
            symbol: name.into() 
        }
    }
    pub fn new_user(&mut self, handle: String) -> OrgUser {
        let ou = OrgUser::new(handle.clone());
        self.users.push(OrgUser::new(handle.clone()));
        return ou;
    }
    pub fn get_users_mut(&mut self) -> Vec<OrgUser> {
        return self.users.clone();
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
    pub fn get_or_create_user(mut self, handle: String) -> OrgUser {
        let users = self.users.to_vec();
        match users.into_iter().find(|u| u.handle == handle.clone()) {
            Some(u) => return u,
            None => {
                let u = OrgUser::new(handle.clone());
                self.users.push(OrgUser::new(handle));
                return u;
            },
        }
    }
    pub fn new_from(
        name: &str, 
        identifier: &str,
        symbol: &str,
        users: Vec<OrgUser>,
    ) -> Self {
        Self { 
            id: OrgId::new(),
            name: name.into(), 
            identifier: identifier.into(), 
            users: Vec::new(),
            symbol: symbol.into(), 
        }
    }
}

impl fmt::Display for Org {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


