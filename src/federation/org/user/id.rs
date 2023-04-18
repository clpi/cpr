use super::{super::Org, OrgId};
use crate::models::{ident::Id, HasIdentifier};
use serde::{Deserialize, Serialize};
use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use super::OrgUser;

use super::super::{FedId, Federation};
use rand::{distributions::Alphanumeric, Rng, RngCore};
pub const ORG_USER_DISCRIMINATOR: &str = "OU";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd)]
#[serde()]
pub struct OrgUserId {
    pub id: String,
    pub handle: String,
    #[serde()]
    pub org_id: OrgId,
}
impl Id<OrgUser> for OrgUserId {
    fn id(&self) -> String {
        self.id.clone()
    }
    fn handle(&self) -> String {
        self.handle.clone()
    }
}
impl HasIdentifier<OrgUserId> for OrgUser {
    type Previous = Org;
    type PreviousId = OrgId;

    fn get_from_id(id: OrgUserId) -> anyhow::Result<Self> {
        Ok(Self::default())
    }
    fn get_id(&self) -> String {
        self.id.clone().id
    }

    fn get_parent(&self) -> Self::Previous {
        // Org::lookup(self.id.org_id.clone())
        Org::default()
    }

    fn get_discriminator() -> String {
        ORG_USER_DISCRIMINATOR.to_string()
    }
    fn get_discriminator_len() -> usize {
        ORG_USER_DISCRIMINATOR.len()
    }
    fn get_global_ident(&self) -> String {
        format!(
            "{} {}",
            self.get_parent_inclusive_ident(),
            self.get_local_ident()
        )
    }
    fn get_handle(&self) -> String {
        self.id.clone().handle
    }
    fn get_parent_identifier(&self) -> Self::PreviousId {
        self.id.clone().org_id
    }
    fn get_local_ident_len_range() -> (usize, usize) {
        (1, 13)
    }
    fn get_local_ident(&self) -> String {
        self.id.clone().to_string()
    }
    fn get_parent_inclusive_ident(&self) -> String {
        format!("{} {}", self.id.clone().org_id.to_string(), self.id.clone().to_string())
    }
    fn get_global_ident_len_range() -> (usize, usize) {
        let (fmin, fmax) = Federation::get_local_ident_len_range();
        let (min, max) = OrgUser::get_parent_inclusive_ident_len_range();
        (fmin + min, fmax + max)
    }
    fn get_parent_inclusive_ident_len_range() -> (usize, usize) {
        let (pmin, pmax) = Self::Previous::get_local_ident_len_range();
        let (min, max) = OrgUser::get_local_ident_len_range();
        (pmin + min, pmax + max)
    }

    fn get_cumulative_ident(&self) -> String {
        return self.clone().get_global_identifier();
    }
    fn get_parent_identifier_str(&self) -> String {
        self.id.clone().org_id.to_string()
    }
    fn get_id_len() -> usize {
        2
    }
    /// Local ident range is thus 1 + 3 + 2 + 2 = 8 min
    /// to 15 + 3 + 2 + 2 = 22 -> (8, 22)
    fn get_handle_len_range() -> (usize, usize) {
        (2, 16)
    }
    fn get_parent_id(&self) -> Option<String> {
        Some(self.id.clone().org_id.id)
    }
    fn get_parent_name(&self) -> Option<String> {
        Some(self.id.clone().org_id.handle)
    }
}
impl Default for OrgUserId {
    fn default() -> Self {
        let id = <OrgUser as HasIdentifier<OrgUserId>>::gen_new_id();
        Self {
            id: id.clone(),
            handle: id.to_string(),
            org_id: OrgId::default(),
        }
    }
}

impl OrgUserId {
    pub fn new(org_id: OrgId, handle: String) -> OrgUserId {
        OrgUserId {
            handle,
            org_id,
            ..OrgUserId::default()
        }
    }
}
// impl Deref for OrgUserId {
    // type Target = String;
    // fn deref(&self) -> &Self::Target {
        // &self.id.clone().to_string()
    // }
// }
// impl DerefMut for OrgUserId {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.to_string()
//     }
// }
impl ToString for OrgUserId {
    fn to_string(&self) -> String {
        format!(
            "{}:{}:{};",
            self.handle,
            <OrgUser as HasIdentifier<OrgUserId>>::get_discriminator(),
            self.id,
        )
    }
}
impl FromStr for OrgUserId {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() >= OrgUser::get_global_ident_len_range().1 {
            return Err(anyhow::anyhow!("Greater than max feasible ident range"));
        } else if s.len() < OrgUser::get_local_ident_len_range().0 {
            return Err(anyhow::anyhow!(
                "Smaller than lowest possible local ident range"
            ));
        } else if s.len() <= OrgUser::get_local_ident_len_range().1 {
            let mut spl = s.split(';').collect::<Vec<&str>>();
            let mut users = spl[0].split(':').collect::<Vec<&str>>();
            return Ok(OrgUserId {
                id: users[2].into(),
                org_id: OrgId::default(),
                handle: users[0].into(),
            });
        } else if s.len() <= OrgUser::get_parent_inclusive_ident_len_range().1 {
            if s.len() >= OrgUser::get_parent_inclusive_ident_len_range().0 {
                let mut spl = s.split(';').collect::<Vec<&str>>();
                let mut orgs = spl[0].split(':').collect::<Vec<&str>>();
                let mut users = spl[1].split(':').collect::<Vec<&str>>();
                return Ok(OrgUserId {
                    id: users[2].into(),
                    handle: users[0].into(),
                    org_id: OrgId {
                        id: orgs[2].into(),
                        handle: orgs[0].into(),
                        fed_id: FedId::default(),
                    },
                });
            } else {
                return Err(anyhow::anyhow!("Invalid length"));
            }
        } else {
            let mut spl = s.split(';').collect::<Vec<&str>>();
            let mut feds = spl[0].split(':').collect::<Vec<&str>>();
            let mut orgs = spl[1].split(':').collect::<Vec<&str>>();
            let mut users = spl[2].split(':').collect::<Vec<&str>>();
            return Ok(OrgUserId {
                id: users[2].into(),
                handle: users[0].into(),
                org_id: OrgId {
                    id: orgs[2].into(),
                    handle: orgs[0].into(),
                    fed_id: FedId {
                        id: feds[2].into(),
                        handle: feds[0].into(),
                        ..Default::default()
                    },
                },
            });
        }
    }
}

impl Into<String> for OrgUserId {
    fn into(self) -> String {
        self.to_string()
    }
}
