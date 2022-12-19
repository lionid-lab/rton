use crate::constructor::Constructor;
use crate::delimiter::Delimiter;
use crate::field::Field;
use crate::field_type::Type;

#[derive(Debug, Clone)]
pub enum Item {
    Delimiter(Delimiter),
    Constructor(Constructor<Type, Field>),
    Layer(u32),
}
