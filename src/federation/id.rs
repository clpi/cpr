use crate::{models::{HasIdentifier, Balance, ident::Id}, Federation};
use rand::{distributions::Alphanumeric, Rng, RngCore};
use serde::{Deserialize, Serialize};
use std::{
    ops::{Deref, DerefMut},
    str::FromStr, collections::BTreeMap,
};

pub const FEDERATION_DISCRIM: &'static str = "F";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd)]
#[serde()]
pub struct FedId {
    pub id: String,
    pub handle: String,
    pub tags: Vec<String>,
    pub metadata: Option<BTreeMap<String, Vec<String>>>,
}

impl Id<Federation> for FedId {
    fn id(&self) -> String {
        self.id.clone()
    }
    fn handle(&self) -> String {
        self.handle.clone()
    }
}

impl Default for FedId {
    fn default() -> Self {
        let rid = Federation::gen_new_id();
        Self {
            id: rid.clone(),
            handle: rid.to_string(),
            tags: Vec::new(),
            metadata: None,
        }
    }
}

impl FedId {
    pub fn new(handle: String) -> FedId {
        Self {
            id: Federation::gen_new_id(),
            handle, ..Default::default()
        }
    }
}
// impl Deref for FedId {
//     type Target = String;
//     fn deref(&self) -> &Self::Target {
//         &self.to_string()
//     }
// }
// impl DerefMut for FedId {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.to_string()
//     }
// }
/// For the purposes of the ToString trait,
/// a general heuristic is taken up whenever ambiguity
/// of identifier scope is found, such that if the scope
/// is not specified, the locally important id is returned.
/// Only when an ident is asked for more globalal specificity
/// does it actually return. All `HasIdentifier`-implementing
/// data types may return finely-scoped idents upon request.
impl ToString for FedId {
    fn to_string(&self) -> String {
        format!("{}:{}:{};", self.handle, Self::discriminator(), self.id)
    }
}
/// In attempting to construct an ID from an ambiguous String,
/// (the scope is not specified to guide our logic) we will make
/// use of our trait-leveraged range knowledge to infer the most
/// likely ID from the String (if it matches up in terms of content
/// with colons in the right places, it's a very good guess as such.)
impl FromStr for FedId {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < Federation::get_global_ident_len_range().0 {
            return Err(anyhow::anyhow!(
                "Generated Federation ID must be > 16 chars"
            ));
        } else if s.len() > Federation::get_global_ident_len_range().1 {
            return Err(anyhow::anyhow!(
                "Generated Federation ID must be < 18 chars"
            ));
        } else {
            let mut spl = s.split(';').collect::<Vec<&str>>();
            let mut feds = spl[0].split(':').collect::<Vec<&str>>();
            return Ok(FedId{
                id: feds[2].to_string(),
                handle: feds[0].to_string(),
                ..Default::default()
            })
        }
    }
}

/// We considerr Strings inputted of a length lower than could be possible
/// from a global minimal length identifier instantly disqualified as parseable
/// ident strings, also those where an identifier scope breaks one of their, or
/// in combination, our, scope's specific length rules.
impl TryFrom<String> for FedId {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl Into<String> for FedId {
    fn into(self) -> String {
        self.to_string()
    }
}

impl HasIdentifier<FedId> for Federation {

    type Previous = Federation;
    type PreviousId = FedId;

    fn get_parent(&self) -> Self::Previous {
        self.clone()
    }
    fn get_parent_id(&self) -> Option<String> {
        None
    }
    fn get_parent_name(&self) -> Option<String> {
        None
    }
    fn get_cumulative_ident(&self) -> String {
        self.get_local_ident()
    }
    fn get_parent_identifier(&self) -> Self::PreviousId {
        self.id.clone()
    }

    fn get_discriminator_len() -> usize {
        Self::get_discriminator().len()
    }
    fn get_discriminator() -> String {
        FEDERATION_DISCRIM.to_string()
    }

    fn get_id(&self) -> String {
        self.id.clone().id
    }
    fn get_handle(&self) -> String {
        self.id.clone().handle
    }
    /// Federations always have a secret 3-digit (for now) code
    fn get_id_len() -> usize {
        2
    }
    /// Federations may have between 3-15 character handles
    /// meaning the range of acceptable Federation global idents
    /// is 6-18, all factors taken into account
    fn get_handle_len_range() -> (usize, usize) {
        (2, 16)
    }
    /// Local ident len range is (4, 18)
    fn get_parent_inclusive_ident_len_range() -> (usize, usize) {
        Self::get_local_ident_len_range()
    }
    /// The lowest identifying basis, so local identifier
    /// is equivalent to global identifier for a Federation
    fn get_global_ident_len_range() -> (usize, usize) {
        Self::get_local_ident_len_range()
    }
    /// Nothing to be cumulative at ground level
    fn get_parent_inclusive_ident(&self) -> String {
        self.get_local_ident()
    }
    /// Nothing to take into account globally
    fn get_global_ident(&self) -> String {
        self.get_cumulative_ident()
    }
}

