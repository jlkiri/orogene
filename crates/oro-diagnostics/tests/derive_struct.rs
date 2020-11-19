use oro_diagnostics::Diagnostic;
use oro_diagnostics::DiagnosticCategory;
use oro_diagnostics_derive::Diagnostic;
use thiserror::Error;

#[derive(Diagnostic, Debug, Error)]
#[error("Useless garbage.")]
#[label("useless::struct")]
#[advice("This struct is useless.")]
pub struct Useless {
    field: i32,
}

#[test]
fn it_works() {
    let usl = Useless { field: 1 };
    assert_eq!("useless::struct", usl.label());
    assert_eq!("This struct is useless.", usl.advice().unwrap());
}
