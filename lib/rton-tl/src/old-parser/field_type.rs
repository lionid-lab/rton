use crate::field::Field;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Int,
    Flags,
    Named(Vec<String>),
    TypeParameter(String),
    Generic(Vec<String>, Box<Type>),
    Flagged(String, u32, Box<Type>),
    Repeated(Option<u32>, Vec<Field>),
}
