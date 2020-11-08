use oro_diagnostics::{Diagnostic, DiagnosticCode};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric0;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::combinator::cut;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::error::{ErrorKind, ParseError};
use nom::multi::many0;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::Err;
use nom::Err::Error;
use nom::IResult;

use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
enum Value {
    String(String),
    Integer(i32),
    Float(f64),
    Boolean(bool),
    Null,
}

#[derive(Debug, thiserror::Error)]
pub enum SDLangParseError<I: Debug> {
    #[error("{0:#?}: Nom internal error: {2:?}.")]
    Nom(DiagnosticCode, I, ErrorKind),
    #[error("{0:#?}: Rust runtime error.")]
    RustRuntimeError(DiagnosticCode, I),
    #[error("{0:#?}: Expect tag.")]
    AbsentIdentifier(DiagnosticCode, I),
    #[error("{0:#?}: Illegal character in the identifier.")]
    IllegalIdentifier(DiagnosticCode, I),
}

type ParseResult<I, T> = IResult<I, T, SDLangParseError<I>>;

impl<'a> From<(&'a str, ErrorKind)> for SDLangParseError<&'a str> {
    fn from((i, ek): (&'a str, ErrorKind)) -> Self {
        SDLangParseError::Nom(DiagnosticCode::OR1000, i, ek)
    }
}

impl<I: Debug> ParseError<I> for SDLangParseError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        SDLangParseError::Nom(DiagnosticCode::OR1000, input, kind)
    }
    fn append(_input: I, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<I: Sync + Send + Debug> Diagnostic for SDLangParseError<I> {
    fn code(&self) -> DiagnosticCode {
        use SDLangParseError::*;
        match self {
            RustRuntimeError(code, _) => *code,
            AbsentIdentifier(code, _) => *code,
            IllegalIdentifier(code, _) => *code,
            _ => DiagnosticCode::OR1000,
        }
    }
}

fn skip_any_space<'a, F: 'a, O, E: ParseError<&'a str>>(
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    preceded(multispace0, f)
}

fn space<'a>(source: &'a str) -> IResult<&'a str, &'a str, SDLangParseError<&'a str>> {
    take_while1(|c: char| c == ' ')(source)
}

fn skip_space<'a, F: 'a, O>(
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, SDLangParseError<&'a str>>
where
    F: Fn(&'a str) -> IResult<&'a str, O, SDLangParseError<&'a str>>,
{
    terminated(f, space)
}

fn integers(source: &str) -> ParseResult<&str, Vec<Value>> {
    many0(skip_space(integer))(source)
}

fn floats(source: &str) -> ParseResult<&str, Vec<Value>> {
    many0(skip_space(float))(source)
}

fn integer(source: &str) -> ParseResult<&str, Value> {
    let (input, raw) = recognize(pair(opt(tag("-")), digit1))(source).map_err(|e| e)?;
    match str::parse::<i32>(raw) {
        Err(_) => Err(Error(SDLangParseError::RustRuntimeError(
            DiagnosticCode::OR1000,
            input,
        ))),
        Ok(int) => Ok((input, Value::Integer(int))),
    }
}

fn float(source: &str) -> ParseResult<&str, Value> {
    let (input, raw) =
        recognize(tuple((opt(tag("-")), digit1, tag("."), digit1)))(source).map_err(|e| e)?;

    match str::parse::<f64>(raw) {
        Err(_) => Err(Error(SDLangParseError::RustRuntimeError(
            DiagnosticCode::OR1000,
            input,
        ))),
        Ok(f) => Ok((input, Value::Float(f))),
    }
}

fn identifier_rest(source: &str) -> ParseResult<&str, &str> {
    recognize(many0(alt((alpha1, tag("_"), tag("$"), tag("-")))))(source)
}

fn identifier(source: &str) -> ParseResult<&str, Value> {
    let (input, id) =
        recognize(pair(opt(tag("_")), identifier_rest))(source).map_err(Err::convert)?;

    if id.is_empty() {
        return Err(Error(SDLangParseError::AbsentIdentifier(
            DiagnosticCode::OR1000,
            input,
        )));
    }

    if !input.is_empty() {
        return Err(Error(SDLangParseError::IllegalIdentifier(
            DiagnosticCode::OR1000,
            input,
        )));
    }

    Ok((input, Value::String(String::from(id))))
}

fn string_body(source: &str) -> ParseResult<&str, &str> {
    alphanumeric0(source)
}

fn string(source: &str) -> ParseResult<&str, Value> {
    let (input, id) = preceded(tag("\""), cut(terminated(string_body, tag("\""))))(source)
        .map_err(Err::convert)?;

    Ok((input, Value::String(String::from(id))))
}

fn boolean(source: &str) -> ParseResult<&str, Value> {
    alt((
        value(Value::Boolean(true), tag("true")),
        value(Value::Boolean(false), tag("false")),
    ))(source)
}

fn null(source: &str) -> ParseResult<&str, Value> {
    value(Value::Null, tag("null"))(source)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_ok(r: ParseResult<&'static str, Value>) -> Value {
        match r {
            Ok((_, v)) => v,
            Err(e) => panic!(e),
        }
    }

    fn assert_ok_with_input(r: ParseResult<&'static str, Value>) -> (&str, Value) {
        match r {
            Ok(t) => t,
            Err(e) => panic!(e),
        }
    }

    #[test]
    fn parse_float() {
        let value = assert_ok(float("5.678"));

        assert_eq!(value, Value::Float(5.678));
    }

    #[test]
    fn parse_integer() {
        let value = assert_ok(integer("578"));

        assert_eq!(value, Value::Integer(578));
    }

    #[test]
    fn parse_identifier() {
        let value = assert_ok(identifier("author"));

        assert_eq!(value, Value::String(String::from("author")));
    }

    #[test]
    fn parse_string() {
        let value = assert_ok(string("\"svelte\""));

        assert_eq!(value, Value::String(String::from("svelte")));
    }

    #[test]
    fn parse_boolean() {
        let truthy = assert_ok(boolean("true"));
        let falsey = assert_ok(boolean("false"));

        assert_eq!(truthy, Value::Boolean(true));
        assert_eq!(falsey, Value::Boolean(false));
    }

    #[test]
    fn parse_null() {
        let null_value = assert_ok(null("null"));

        assert_eq!(null_value, Value::Null);
    }

    #[test]
    fn skip_any_space_around() {
        let value = assert_ok(skip_any_space(string)("  \n  \"svelte\""));

        assert_eq!(value, Value::String(String::from("svelte")));
    }

    #[test]
    fn skip_space_after() {
        let (input, value) = assert_ok_with_input(skip_space(string)("\"svelte\"    "));

        assert!(input.is_empty());
        assert_eq!(value, Value::String(String::from("svelte")));
    }
}
