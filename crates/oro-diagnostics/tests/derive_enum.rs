use oro_diagnostics::Diagnostic;
use oro_diagnostics::DiagnosticCategory;
use oro_diagnostics_derive::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Useless garbage.")]
pub enum Useless {
    #[label("useless::garbage")]
    #[advice("Garbage.")]
    Garbage,
    #[label("useless::pineapple_pizza")]
    #[advice("Don't.")]
    PineapplePizza,
}

#[test]
fn it_works() {
    let gbg = Useless::Garbage;
    assert_eq!("Garbage.", gbg.advice().unwrap());
    assert_eq!("useless::garbage", gbg.label());

    let pp = Useless::PineapplePizza;
    assert_eq!("Don't.", pp.advice().unwrap());
    assert_eq!("useless::pineapple_pizza", pp.label());
}
