use oro_diagnostics::{Diagnostic, DiagnosticCode};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::alpha0;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::error::FromExternalError;
use nom::error::ParseError;
use nom::multi::count;
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::tuple;
use nom::IResult;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0:#?}: {1}.")]
    RustError(DiagnosticCode, String),
    #[error("{0:#?}: Tag name not found.")]
    Identifier(DiagnosticCode),
}

impl Diagnostic for Error {
    fn code(&self) -> DiagnosticCode {
        use Error::*;
        match self {
            RustError(code, _) => *code,
            Identifier(code) => *code,
        }
    }
}

#[derive(Debug)]
enum Value {
    String(String),
    Integer(i32),
    Float(f64),
    Boolean(bool),
    Null,
}

fn space_around<'a, F: 'a, O, E: ParseError<&'a str>>(
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, f, multispace0)
}

fn integers(source: &str) -> IResult<&str, Vec<Value>> {
    many0(space_around(integer))(source)
}

fn floats(source: &str) -> IResult<&str, Vec<Value>> {
    many0(space_around(float))(source)
}

fn integer<'a, E>(source: &'a str) -> IResult<&'a str, Value, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, Error>,
{
    map_res(recognize(pair(opt(tag("-")), digit1)), |raw| {
        let int = str::parse::<i32>(raw)
            .map_err(|e| Error::RustError(DiagnosticCode::OR1000, format!("{}", e)))?;
        Ok(Value::Integer(int))
    })(source)
}

fn float<'a, E>(source: &'a str) -> IResult<&'a str, Value, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, Error>,
{
    map_res(
        recognize(tuple((opt(tag("-")), digit1, tag("."), digit1))),
        |raw| {
            let float = str::parse::<f64>(raw)
                .map_err(|e| Error::RustError(DiagnosticCode::OR1000, format!("{}", e)))?;
            Ok(Value::Float(float))
        },
    )(source)
}

fn identifier_part<'a, E: ParseError<&'a str>>(source: &'a str) -> IResult<&'a str, &str, E> {
    recognize(many0(alt((alpha1, tag("_"), tag("$"), tag("-")))))(source)
}

fn identifier(source: &str) -> IResult<&str, Value> {
    consume_identifier(source)
}

fn consume_identifier<'a, E>(source: &'a str) -> IResult<&'a str, Value, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, Error>,
{
    map_res(
        recognize(pair(opt(tag("_")), identifier_part)),
        |res: &str| {
            if res.is_empty() {
                return Err(Error::Identifier(DiagnosticCode::OR1000));
            }

            Ok(Value::String(String::from(res)))
        },
    )(source)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        /* println!("{:?}", integers("033 256\n123"));
        println!("{:?}", identifier("___abc-de$f")); */
        println!("{:#?}", floats("25.455 1344.5"));
        println!("{:?}", identifier(""));
        assert_eq!(2 + 2, 4);
    }
}
