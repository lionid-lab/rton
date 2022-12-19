use crate::field_type::Type;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Field {
    pub name: Option<String>,
    pub ty: Type,
}
