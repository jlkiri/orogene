use oro_diagnostics::Diagnostic;
use oro_diagnostics::DiagnosticCategory;
use oro_diagnostics_derive::Diagnostic;
use thiserror::Error;

#[derive(Diagnostic, Debug, Error)]
#[error("Colored struct.")]
#[label("color::struct")]
#[advice("Color.")]
#[category(Misc)]
pub struct Color;

#[test]
fn it_works() {
    let clr = Color {};
    assert_eq!("color::struct", clr.label());
    assert_eq!("Color.", clr.advice().unwrap());
}
