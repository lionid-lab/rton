use super::constants::*;
use super::tag;
use super::utils::utf8;
use crate::parser::tag::Tag;
use pom::char_class::{alphanum, digit};
use pom::parser::Parser;
use pom::parser::*;

fn decimal<'a>() -> Parser<'a, u8, u32> {
    is_a(digit).repeat(1..).convert(|s| utf8(s).parse())
}

pub fn ident<'a>() -> Parser<'a, u8, String> {
    (is_a(alphanum) | sym(b'_')).repeat(1..).map(utf8)
}

fn dotted_ident<'a>() -> Parser<'a, u8, String> {
    ((ident() - sym(DOT)).repeat(0..) + ident())
        .map(|(mut v, i)| {
            v.push(i);
            v
        })
        .map(|v| v.join(DOT.to_string().as_str()))
}

// fn combinator_declaration<'a>() -> Parser<'a, char, char> {
//     one_of("todo")
// }

pub fn constructor<'a>() -> Parser<'a, u8, (String, Option<Tag>)> {
    constructor_name() + tag::tag()
}

fn constructor_name<'a>() -> Parser<'a, u8, String> {
    dotted_ident()
}

// TODO

// fn anonymous_constructor<'a>() -> Parser<'a, u8, Vec<(String, String)>> {
//     sym(LBRACKET) * field_definitions() - sym(RBRACE)
// }

// fn paren_expression<'a>() -> Parser<'a, u8, (String, String)> {
//     sym(LPAREN) * expression() - sym(RPAREN)
// }

// fn expression<'a>() -> Parser<'a, u8, (String, String)> {
//     expr95()
// }

// fn expr95<'a>() -> Parser<'a, u8, (String, String)> {
//     todo!()
// }

// fn type_expression<'a>() -> Parser<'a, u8, (String, String)> {
//     sym(TILDE).opt() + (paren_expression() | anonymous_constructor() | cell_ref() | builtin_type() | decimal() | named_ref())
// }

// fn named_ref<'a>() -> Parser<'a, u8, String> {
//     ident()
// }
