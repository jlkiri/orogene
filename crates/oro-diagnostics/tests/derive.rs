use oro_diagnostics::Diagnostic;
use oro_diagnostics::DiagnosticCategory;
use oro_diagnostics_derive::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Useless garbage.")]
pub enum Useless {
    #[advice("Garbage.")]
    Garbage,
    #[advice("Don't.")]
    PineapplePizza,
}

#[test]
fn it_works() {
    let gbg = Useless::Garbage;
    assert_eq!("Garbage.", gbg.advice().unwrap());

    let pp = Useless::PineapplePizza;
    assert_eq!("Don't.", pp.advice().unwrap());
}
