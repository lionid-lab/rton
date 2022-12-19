use crate::parser::tag::Tag;
use crate::parser::tokens::constructor;
use pom::parser::Parser;

mod constants;
mod field;
mod field_type;
mod tag;
mod tokens;
mod utils;

pub fn parse_string<'a>(input: &'a str) -> pom::Result<(String, Option<Tag>)> {
    let mut input: &'a [u8] = input.as_bytes();
    grammar().parse(&mut input)
}

pub fn grammar<'a>() -> Parser<'a, u8, (String, Option<Tag>)> {
    constructor()
}
