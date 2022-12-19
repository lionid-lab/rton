mod tokens;
mod utils;

use pom::char_class::{alphanum, digit, hex_digit};
use pom::parser::*;
use crate::constructor::Constructor;
use crate::field::Field;
use crate::field_type::Type;
use crate::item::Item;
use crate::delimiter::Delimiter;

pub fn parse_string<'a>(input: &'a str) -> pom::Result<Vec<String>> {
    let mut input: &'a [u8] = input.as_bytes();
    grammar().parse(&mut input)
}

pub fn grammar<'a>() -> Parser<'a, u8, Vec<String>> {
    lines()
}

pub fn lines<'a>() -> Parser<'a, u8, Vec<String>> {
    space() * item().repeat(0..)
}

fn space<'a>() -> Parser<'a, u8, ()> {
    let end_comment = || seq(b"*/");
    (
        one_of(b" \t\r\n").discard() |
            (
                seq(b"//") - !(seq(b" LAYER ")) - none_of(b"\r\n").repeat(0..)
            ).discard() |
            (
                seq(b"/*") * (!end_comment() * take(1)).repeat(0..) * end_comment()
            ).discard()
    ).repeat(0..).discard()
}

fn item<'a>() -> Parser<'a, u8, Item> {
    (delimiter().map(Item::Delimiter) |
        constructor().map(Item::Constructor) |
        layer().map(Item::Layer)) - space()
}

fn constructor() -> Parser<u8, Constructor<Type, Field>> {
    (dotted_ident() + tl_id().opt() + fields() - simple_space() - sym(b'=') - simple_space() + ty_space_generic() - sym(b';'))
        .map(|(((variant, tl_id), (type_parameters, fields)), output)| {
            Constructor {
                tl_id: get_tl_id(tl_id, variant.1.clone(), type_parameters.clone(), fields.clone(), output.1.clone()),
                type_parameters,
                fields,
                original_variant: variant.join(""),
                variant: Type::Named(variant),
                original_output: output.1,
                output: output.0,
                is_function: false,
            }
        })
        .name("constructor")
}

fn delimiter<'a>() -> Parser<'a, u8, Delimiter> {
    seq(b"---types---").map(|_| Delimiter::Types) |
        seq(b"---functions---").map(|_| Delimiter::Functions)
}

fn layer<'a>() -> Parser<'a, u8, u32> {
    seq(b"// LAYER ") * decimal()
}

fn decimal<'a>() -> Parser<'a, u8, u32> {
    is_a(digit).repeat(0..).convert(|s| utf8(s).parse())
}

fn utf8<'a>(v: Vec<u8>) -> String {
    String::from_utf8(v).unwrap()
}

fn dotted_ident() -> Parser<u8, Vec<String>> {
    ((ident() - sym(b'.')).repeat(0..) + ident())
        .map(|(mut v, i)| {
            v.push(i);
            v
        })
}

fn simple_space() -> Parser<u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

fn ident() -> Parser<u8, String> {
    (is_a(alphanum) | sym(b'_')).repeat(1..).map(utf8)
}

fn tl_id() -> Parser<u8, u32> {
    sym(b'#') * is_a(hex_digit).repeat(0..9).convert(|s| u32::from_str_radix(&utf8(s), 16))
}

fn fields() -> Parser<u8, (Vec<Field>, Vec<Field>)> {
    (simple_space().opt() * sym(b'?')).map(|_| (vec![], vec![])) |
        (simple_space() * ty_param_field()).repeat(0..) + base_fields()
}

fn ty_param_field() -> Parser<u8, Field> {
    sym(b'{') * base_field() - sym(b'}')
}

fn base_fields() -> Parser<u8, Vec<Field>> {
    (simple_space() * base_field_anonymous_or_repeated()).repeat(0..)
}

fn base_field_anonymous_or_repeated() -> Parser<u8, Field> {
    repeated_field() |
        base_field() |
        ty().map(|ty| Field { name: None, ty })
}

fn repeated_field() -> Parser<u8, Field> {
    ((decimal() - simple_space() * sym(b'*') * simple_space()).opt()
        + sym(b'[')
        * call(base_fields)
        - seq(b" ]"))
        .map(|(repeat_count, fv)|
            Field { name: None, ty: Type::Repeated(repeat_count, fv) }
        )
}

fn base_field() -> Parser<u8, Field> {
    (ident() - sym(b':') + ty())
        .map(|(name, ty)| Field { name: Some(name), ty })
        .name("field")
}

fn ty() -> Parser<u8, Type> {
    sym(b'#').map(|_| Type::Int) |
        sym(b'!') * ident().map(Type::TypeParameter) |
        ty_flag() |
        ty_generic() |
        dotted_ident().map(Type::Named)
}

fn ty_flag() -> Parser<u8, Type> {
    (ident() - sym(b'.') + decimal() - sym(b'?') + call(ty))
        .map(|((name, bit), ty)| Type::Flagged(name, bit, Box::new(ty)))
}

fn ty_generic() -> Parser<u8, Type> {
    (sym(b'(') * dotted_ident() - simple_space() + call(ty) - sym(b')')).map(|(name, ty)| Type::Generic(name, Box::new(ty))) |
        (dotted_ident() - sym(b'<') + call(ty) - sym(b'>')).map(|(name, ty)| Type::Generic(name, Box::new(ty)))
}

fn ty_space_generic() -> Parser<u8, Type> {
    let space_generic = dotted_ident() - simple_space() + ty();
    space_generic.map(|(name, ty)| Type::Generic(name, Box::new(ty))) |
        ty()
}
