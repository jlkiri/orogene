use oro_diagnostics::{Diagnostic, DiagnosticCode};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::combinator::all_consuming;
use nom::combinator::consumed;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::error::VerboseError;
use nom::error::{ErrorKind, FromExternalError, ParseError};
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::tuple;
use nom::Err;
use nom::Err::Error;
use nom::IResult;
use nom::Parser;

use std::fmt::Debug;

#[derive(Debug)]
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
            _ => DiagnosticCode::OR1000,
        }
    }
}

fn space_around<'a, F: 'a, O, E: ParseError<&'a str>>(
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, f, multispace0)
}

/* fn integers(source: &str) -> IResult<&str, Vec<Value>> {
    many0(space_around(integer))(source)
} */

/* fn floats(source: &str) -> IResult<&str, Vec<Value>> {
    many0(space_around(float))(source)
} */

/* fn integer<'a>(source: &'a str) -> ParseResult<&'a str, Value> {
    let res = recognize(pair(opt(tag("-")), digit1));
    map_res(res, |raw| -> Result<Value, SDLangParseError<String>> {
        let int = str::parse::<i32>(raw).map_err(|e| {
            SDLangParseError::RustRuntimeError(DiagnosticCode::OR1000, format!("{}", e))
        })?;
        Ok(Value::Integer(1))
    })(source)
} */

fn float(source: &str) -> ParseResult<&str, Value> {
    let (input, raw) =
        recognize(tuple((opt(tag("-")), digit1, tag("."), digit1)))(source).map_err(|e| e)?;

    match str::parse::<f64>(raw) {
        Err(e) => Err(Error(SDLangParseError::RustRuntimeError(
            DiagnosticCode::OR1000,
            "",
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

    Ok((input, Value::String(String::from(id))))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_identifier() {
        /* println!("{:?}", integers("033 256\n123"));
        println!("{:?}", identifier("___abc-de$f")); */
        /*  println!("{:#?}", floats("25.455 1344.5"));*/
        println!("{:#?}", identifier("aaa"));

        /* let e = identifier::<SDLangParseError<&str>>("").map_err(|err| match err {
            nom::Err::Error(e) => e,
            nom::Err::Failure(e) => e,
            nom::Err::Incomplete(_) => unreachable!(),
        }); */

        assert_eq!(2 + 2, 4);
    }
}
