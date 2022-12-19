use super::constants::*;
use super::utils::utf8;
use pom::char_class::hex_digit;
use pom::parser::Parser;
use pom::parser::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tag {
    Hex(u32),
    Binary(u32),
    Generated(u32),
    Empty,
}

fn hex_tag<'a>() -> Parser<'a, u8, Tag> {
    sym(TAG)
        * is_a(hex_digit)
            .repeat(0..9)
            .convert(|s| u32::from_str_radix(&utf8(s), 16))
            .map(Tag::Hex)
}

fn binary_tag<'a>() -> Parser<'a, u8, Tag> {
    sym(DOLLAR)
        * one_of(BINARY)
            .repeat(0..9)
            .convert(|s| u32::from_str_radix(&utf8(s), 2))
            .map(Tag::Binary)
}

fn empty_tag<'a>() -> Parser<'a, u8, Tag> {
    (one_of(TAG_SIGN).opt() + sym(UNDERSCORE)).map(|_| Tag::Empty)
}

pub fn tag<'a>() -> Parser<'a, u8, Option<Tag>> {
    (hex_tag() | binary_tag() | empty_tag()).opt()
}
