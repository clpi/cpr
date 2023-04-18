use crate::Federation;
use rand::{distributions::Alphanumeric, Rng, RngCore};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Debug, self},
    ops::{Deref, DerefMut},
    str::FromStr,
};

pub trait Id<T> 
where
    T: HasIdentifier<Self>,
    Self: Serialize 
    + FromStr 
    + ToString 
    + Default 
    + Debug 
    + Clone,
{
    fn id(&self) -> String;
    fn handle(&self) -> String;
    fn discriminator() -> String {
        T::get_discriminator()
    }
    fn local_ident(&self) -> String {
        format!("{}:{}:{};", 
            self.handle(),
            Self::discriminator(),
            self.id(),
        )
    }
    fn handle_len_range() -> (usize, usize) {
        T::get_handle_len_range()
    }
    fn id_len() -> usize {
        T::get_id_len()
    }
    fn discriminator_len() -> usize {
        T::get_discriminator_len()
    }
    fn local_ident_len_range() -> (usize, usize) {
        let (min, max) = Self::handle_len_range();
        let idlen = Self::id_len();
        let discrlen = Self::discriminator_len();
        return (min + 1 + discrlen + 1 + idlen + 1,
                max + 1 + discrlen + 1 + idlen + 1)

    }
}

/// Self: TThe type represented by the ID
/// I: The ID type
pub trait HasIdentifier<I>
where
    Self: Clone + Default + Debug,
    I: Default 
    + ToString 
    + FromStr 
    + Serialize,
{
    /// The wrapped ID type  which directly follows the classification representated
    /// by the ID type
    type Previous: HasIdentifier<Self::PreviousId>;
    ///
    type PreviousId: Id<Self::Previous>;

    /// Get a result-wrapped full type from an ID as an
    /// exchange with the stream-based DAG-runtime
    fn get_from_id(id: I) -> anyhow::Result<Self> {
        Ok(Self::default())
    }

    /// A unique identifier which prefixes the random id in an
    /// identifier string, which is the same for all objects of that
    /// class/type. (Needs more thought) Could be a letter or a word,
    /// or anything to distinguish between different types of objects
    /// allowing for distinct identifier combination strings.
    fn get_discriminator() -> String;

    /// Get the length of the discriminator
    fn get_discriminator_len() -> usize {
        Self::get_discriminator().len()
    }

    /// Default placeholder rng impl
    fn gen_new_id() -> String {
        let mut rng = rand::thread_rng();
        let len = <Self as HasIdentifier<I>>::get_id_len();
        let rid: String = (0..len)
            .map(|_| (&mut rng).sample(Alphanumeric) as char)
            .collect::<String>();
        return rid;
    }

    /// Return the larger federation an org is a part of,
    /// or the larger org a user is part of.
    fn get_parent(&self) -> Self::Previous;

    /// Return the constructed identifier string of the parent
    /// object, or in the case of no parent, return None.
    fn get_parent_identifier(&self) -> Self::PreviousId;


    /// Get cumulative identifier string, if applicable
    fn get_cumulative_ident(&self) -> String;

    ///
    fn get_parent_identifier_str(&self) -> String {
        self.get_parent_identifier().to_string()
    }

    /// Return the generated ID component of the parent object
    /// if it exists, otherwise return None.
    fn get_parent_id(&self) -> Option<String>;

    /// Return the chosen parent handle if it exists, otherwise
    /// return None.
    fn get_parent_name(&self) -> Option<String>;

    /// The identifier which uniquely marks an implementing
    /// type across all federations`or other possible lowest bases.
    fn get_global_ident(&self) -> String;

    /// Get their combined global identifier down to their
    /// position of specificity as an identifier string.
    fn get_parent_inclusive_ident(&self) -> String;


    /// Get their combined identifier String which is only
    /// necessarily as specific as their current parent bound,
    /// i.e. the organization.
    fn get_local_ident(&self) -> String {
        format!("{}:{}", self.get_handle(), self.get_id())
    }

    /// Just returns the local identifier string (delete)
    fn get_ident(&self) -> String {
        self.get_local_ident()
    }

    /// Return the generated component of their ID. Its length
    /// and means of calculation should be statically class-based
    fn get_id(&self) -> String;

    /// Static, class-determined value for how many characters
    /// should be generated for the random component of the ID.
    fn get_id_len() -> usize {
        4
    }
    /// Class static method for class-based handle length limits
    fn get_handle_len_range() -> (usize, usize) {
        (2, 16) // Default between 2-16 characters acceptible
    }

    /// Return their chosen handle, which should fit within the
    /// static, class-determined handle length range above
    fn get_handle(&self) -> String;

    /// Get low and high bounds of local ident bound range
    /// Total local ident is formed: H:DISCR:ID;
    ///     -> H: Handle (range allowed) -> ":"
    ///     -> DISCR: Discriminator (static) -> ":"
    ///     -> ID: Random ID (static) -> ";"
    /// So Hl + DISCRl + IDl + 3 = total length
    fn get_local_ident_len_range() -> (usize, usize) {
        let (l, h) = <Self as HasIdentifier<I>>::get_handle_len_range(); // 
        let id_len = <Self as HasIdentifier<I>>::get_id_len();
        let discr_len = Self::get_discriminator_len();
        (l + id_len + discr_len + 3, h + id_len + discr_len + 3)
    }

    /// Get the calculated total floor and ceiling re: the number
    /// of characters which could be used to make up a valid
    /// identifier at their relative point in the hierarchy
    fn get_parent_inclusive_ident_len_range() -> (usize, usize) {
        let (min, max) = Self::get_local_ident_len_range();
        let (pmin, pmax) = Self::Previous::get_parent_inclusive_ident_len_range();
        (min + pmin + 1, max + pmax + 1)

    }

    /// Get prior range limits, representing those bounds which
    /// determined identifier strings composed for the sake of
    /// their parent organization's identifier strings
    fn get_global_ident_len_range() -> (usize, usize);
}

// pub trait SocialClassifier<G, I>
// where
//     G: Default + Debug + Clone + HasIdentifier<I>,
//     I: Default + FromStr + ToString 
// {
//     type Unit: Id<G>;
//     type UnitId: Id<G>;
//     type Discriminator: Into<String> + FromStr;
// }
//
//
