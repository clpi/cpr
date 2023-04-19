#[derive(Debug, Clone, PartialEq)]
pub struct Validator {
    name: String,
    weight: usize,
    pub key: String,
}
