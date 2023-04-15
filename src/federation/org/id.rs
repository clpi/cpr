use std::ops::Deref;

use serde::{Serialize, Deserialize};
use rand::{Rng, RngCore, distributions::Alphanumeric};

use crate::federation::id::FedId;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialOrd)]
pub struct OrgId{
    pub id: String,
    pub fed_id: FedId,
    pub name: String,
}

impl Default for OrgId {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let rid: String =  (0..16).map(|_| (&mut rng).sample(Alphanumeric) as char)
            .collect::<String>();
        Self {
            id: rid,
            fed_id: FedId::default(),
            name: rid,
        }
    }
}

impl TryFrom<String> for OrgId {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() < 16 {
            Err(anyhow::anyhow!("OrgId must be at least 16 characters long"))
        } else if value.len() < 32 {
            let (org_id, org_name) = value.split_at(16);
            Ok(Self {
                id: id.to_string(),
                fed_id: FedId::none(),
                name: name.to_string(),
            })
        }

        } else {
            let (fed_id, name) = value.split_at(16);
            Ok(Self {
                id: id.to_string(),
                fed_id: FedId::default(),
                name: name.to_string(),
            })
        }
    }
}
impl TryInto<String> for OrgId {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<String, Self::Error> {
        if (self.id.len() + self.name.len()) < 16 {
            Err(anyhow::anyhow!("OrgId must be at least 16 characters long"))
        } else {
            Ok(format!("{}{}", self.id, self.name))
        }
    }
}

impl PartialEq for OrgId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
            self.name == other.name
    }
}

impl OrgId {
    pub fn new(handle: &str) -> OrgId {
        Self {
            name: handle.into(),
            ..Self::default()
        }
    }
    pub fn get_identifier(&self) -> String {
        self.to_string()
    }
    pub fn gen() -> OrgId {
        OrgId::default()
    }
    pub fn get_global_identifier(&self) -> String {
        format!("{}:{}", self.fed_id.to_string(), self.id)
    }
    pub fn get_in_federation_identifier(&self) -> String {
        self.id.to_string()
    }
    
}
impl std::fmt::Display for OrgId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.id, self.name)
    }
}


// impl From<String> for OrgId {
//     fn from(value: String) -> Self {
//         if value.len() < 16 {
//             Self {
//                 id: value,
//                 name: value,
//             }
//         } else {
//             let (id, name) = value.split_at(16);
//             Self {
//                 id: id.to_string(),
//                 name: name.to_string(),
//             }
//         }
//         
//     }
// }
