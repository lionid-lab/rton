use super::constants::*;
use crate::parser::field_type::FieldType;
use crate::parser::tokens::ident;
use pom::parser::Parser;
use pom::parser::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Field {
    Implicit {
        name: Option<String>,
        type_: FieldType,
        original: String,
    },
    Explicit {
        name: Option<String>,
        type_: FieldType,
        original: String,
    },
    CellRef {
        type_: FieldType,
        original: String,
    },
}

fn field_definitions<'a>() -> Parser<'a, u8, Vec<Field>> {
    field_definition().repeat(0..)
}

fn field_definition<'a>() -> Parser<'a, u8, Field> {
    implicit_field_definition() | explicit_field_definition() | cell_ref()
}

// implicit_field_definition
fn implicit_field_definition<'a>() -> Parser<'a, u8, Field> {
    (sym(LBRACE) * implicit_field()) - sym(RBRACE)
}

fn implicit_field<'a>() -> Parser<'a, u8, Field> {
    (implicit_field_name() + sym(COLUMN) + implicit_field_type()).map(|((name, sym), type_)| {
        let original = format!(
            "{}{}{}",
            name.clone().unwrap_or(UNDERSCORE.to_string()),
            sym,
            type_.original
        );
        Field::Implicit {
            name,
            type_,
            original,
        }
    })
}

fn implicit_field_name<'a>() -> Parser<'a, u8, Option<String>> {
    ident().map(Some) | sym(UNDERSCORE).map(|_| None)
}

fn implicit_field_type<'a>() -> Parser<'a, u8, FieldType> {
    // TODO
    any().map(|_| FieldType {
        original: "todo".to_string(),
    })
}

// explicit_field_definition
fn explicit_field_definition<'a>() -> Parser<'a, u8, Field> {
    explicit_field()
}

fn explicit_field<'a>() -> Parser<'a, u8, Field> {
    (explicit_field_name() + sym(COLUMN) + explicit_field_type()).map(|((name, sym), type_)| {
        let original = format!(
            "{}{}{}",
            name.clone().unwrap_or(UNDERSCORE.to_string()),
            sym,
            type_.original
        );
        Field::Explicit {
            name,
            type_,
            original,
        }
    })
}

fn explicit_field_name<'a>() -> Parser<'a, u8, Option<String>> {
    ident().map(Some) | sym(UNDERSCORE).map(|_| None)
}

fn explicit_field_type<'a>() -> Parser<'a, u8, FieldType> {
    // TODO
    any().map(|_| FieldType {
        original: "todo".to_string(),
    })
}

// cell_ref
fn cell_ref<'a>() -> Parser<'a, u8, Field> {
    // TODO
    sym(CIRCUMFLEX) // * type_expression()
        .map(|_| Field::CellRef {
            type_: FieldType {
                original: "todo".to_string(),
            },
            original: "todo".to_string(),
        })
}
