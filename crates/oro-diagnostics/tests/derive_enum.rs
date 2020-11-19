use oro_diagnostics::Diagnostic;
use oro_diagnostics::DiagnosticCategory;
use oro_diagnostics_derive::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Rainbow error.")]
#[label("critical::rainbow")]
#[advice("Rainbow.")]
#[category(Misc)]
pub struct Rainbow;

#[derive(Debug, Error, Diagnostic)]
#[error("Critical error.")]
pub enum Critical {
    #[category(Misc)]
    #[label("critical::blue")]
    #[advice("Blue.")]
    Blue(i32),
    #[label("critical::red")]
    #[advice("Red.")]
    #[category(Misc)]
    Red(i32),
    #[label("critical::orange")]
    #[advice("Orange.")]
    Orange(i32),
    Transparent(#[ask] Rainbow),
}

#[test]
fn it_works() {
    let blue = Critical::Blue(2);
    assert_eq!("Blue.", blue.advice().unwrap());
    assert_eq!("critical::blue", blue.label());
    assert_eq!(DiagnosticCategory::Misc, blue.category());

    let red = Critical::Red(2);
    assert_eq!("Red.", red.advice().unwrap());
    assert_eq!("critical::red", red.label());
    assert_eq!(DiagnosticCategory::Misc, red.category());

    let orange = Critical::Orange(3);
    assert_eq!("Orange.", orange.advice().unwrap());
    assert_eq!("critical::orange", orange.label());
    assert_eq!(DiagnosticCategory::Misc, orange.category());

    let rainbow = Rainbow {};

    let transp = Critical::Transparent(rainbow);
    assert_eq!("Rainbow.", transp.advice().unwrap());
    assert_eq!("critical::rainbow", transp.label());
    assert_eq!(DiagnosticCategory::Misc, transp.category());
}
