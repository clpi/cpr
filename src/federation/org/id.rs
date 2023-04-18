use std::{ops::{Deref, DerefMut}, str::FromStr, mem::discriminant};
use self::super::Org;
use crate::models::ident::Id;
pub use crate::models::Balance;
use serde::{Serialize, Deserialize};
use rand::{Rng, RngCore, distributions::Alphanumeric};
use super::{FedId, Federation, HasIdentifier};

pub const ORG_DISCRIMINATOR: &'static str = "O";

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialOrd)]
#[serde()]
pub struct OrgId {
    pub id: String,
    pub fed_id: FedId,
    pub handle: String,
}

// TODO: Implement Id<OrgUser> for OrgId?
impl Id<Org> for OrgId {
    fn id(&self) -> String {
        self.id.clone()
    }
    fn handle(&self) -> String {
        self.handle.clone()
    }
}

impl HasIdentifier<OrgId> for Org {
    type Previous = Federation;
    type PreviousId = FedId;

    fn get_id(&self) -> String { self.id.clone().id }
    fn get_parent_inclusive_ident(&self) -> String {
        format!("{} {}",
            self.id.clone().fed_id.to_string(),
            self.to_string(),
        )
    }
    fn get_discriminator() -> String {
        ORG_DISCRIMINATOR.to_string()
    }
    fn get_discriminator_len() -> usize { 
        ORG_DISCRIMINATOR.len()
    }
    fn get_parent_identifier(&self) -> Self::PreviousId {
        self.id.clone().fed_id
    }
    fn get_parent(&self) -> Self::Previous {
        Federation::default()
    }
    fn get_parent_id(&self) -> Option<String> {
        Some(self.id.clone().fed_id.id)
    }
    fn get_parent_name(&self) -> Option<String> {
        Some(self.id.clone().fed_id.handle)
    }
    fn get_global_ident_len_range() -> (usize, usize) {
        let (min, max) = Self::get_local_ident_len_range();
        let (fmin, fmax) = Self::get_parent_inclusive_ident_len_range();
        (min + fmin + 1, max + fmax + 1)
    }
    fn get_global_ident(&self) -> String {
        format!("{} {}",
            self.id.clone().fed_id.to_string(),
            self.to_string(),
        )
    }
    fn get_parent_inclusive_ident_len_range() -> (usize, usize) {
        let (min, max) = Self::get_local_ident_len_range();
        let (fmin, fmax) = Self::get_parent_inclusive_ident_len_range();
        (min + fmin + 1, max + fmax + 1)
    }
    fn get_cumulative_ident(&self) -> String {
        format!("{} {}",
            self.id.fed_id.to_string(),
            self.to_string(),
        )
    }
    fn get_handle(&self) -> String { self.id.clone().handle }
    fn get_handle_len_range() -> (usize, usize) {
        (3, 16)
    }
    fn get_id_len() -> usize { 2 }
    
}
impl Default for OrgId {
    fn default() -> Self {
        let rid = Org::gen_new_id();
        Self {
            id: rid.clone(),
            fed_id: FedId::default(),
            handle: rid.clone(),
        }
    }
}
// impl Deref for OrgId {
//     type Target = String;
//     fn deref(&self) -> &Self::Target {
//         &self.id.to_string()
//     }
// }

impl FromStr for OrgId {
    type Err = anyhow::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() < Org::get_local_ident_len_range().0 {
            return Err(anyhow::anyhow!("OrgId must be at least 16 characters long"));
        } else if value.len() >= Org::get_global_ident_len_range().1 {
            return Err(anyhow::anyhow!("GlobalOrgId must be at max characters long"));
        } else if value.len() < Org::get_local_ident_len_range().1 {
            let mut spl = value.split(';').collect::<Vec<&str>>();
            let mut orgs = spl[0].split(':').collect::<Vec<&str>>();
            return Ok(Self {
                id: orgs[2].into(),
                fed_id: FedId::default(),
                handle: orgs[0].into(),
            });
        } else {
            let mut spl = value.split(';').collect::<Vec<&str>>();
            let mut feds = spl[0].split(':').collect::<Vec<&str>>();
            let mut orgs = spl[1].split(':').collect::<Vec<&str>>();
            return Ok(OrgId {
                id: orgs[2].to_string(),
                fed_id: FedId {
                    id: feds[2].to_string(),
                    tags: vec![],
                    metadata: None,
                    handle: feds[0].to_string(),
                },
                handle: orgs[0].to_string(),
            });
        }
        
    }
}

impl ToString for OrgId {
    fn to_string(&self) -> String {
        format!("{}:{}:{};",
            self.handle,
            <Org as HasIdentifier<OrgId>>::get_discriminator(),
            self.id,
        )
    }
}

impl PartialEq for OrgId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
            self.handle == other.handle
    }
}

impl OrgId {

    pub fn new(handle: &str) -> OrgId {
        Self {
            handle: handle.into(),
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
        format!("{} {}", self.fed_id.to_string(), self.to_string())
    }
    pub fn get_in_federation_identifier(&self) -> String {
        self.id.to_string()
    }
    
}
// impl std::fmt::Display for OrgId {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}{}", self.id, self.name)
//     }
// }

impl Into<String> for OrgId {
    fn into(self) -> String {
        self.to_string()
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
