use std::fmt;
use std::path::PathBuf;

use colored::Colorize;
use thiserror::Error;
use url::{Host, Url};

pub struct ErrorMeta {
    net: Option<NetMeta>,
    path: Option<PathMeta>,
    parse_report: Option<ParseMeta>,
}

#[derive(Error)]
#[error("{:?}", self)]
pub struct DiagnosticError {
    pub error: Box<dyn std::error::Error + Send + Sync>,
    pub category: DiagnosticCategory,
    pub label: String,
    pub advice: Option<String>,
    pub meta: Option<ErrorMeta>,
}

impl fmt::Debug for DiagnosticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            return fmt::Debug::fmt(&self.error, f);
        } else {
            use DiagnosticCategory::*;
            write!(f, "{}", self.label.red())?;
            if let Net = &self.category {
                let net_meta = &self.meta.as_ref().unwrap().net;
                if let Some(meta) = net_meta {
                    if let Some(ref url) = meta.url {
                        write!(f, " @ {}", format!("{}", url).cyan().underline())?;
                    } else {
                        write!(f, " @ {}", format!("{}", meta.host).cyan().underline())?;
                    }
                }
            }
            write!(f, "\n\n")?;
            write!(f, "{}", self.error)?;
            if let Some(advice) = &self.advice {
                write!(f, "\n\n{}", "help".yellow())?;
                write!(f, ": {}", advice)?;
            }
        }
        Ok(())
    }
}

pub type DiagnosticResult<T> = Result<T, DiagnosticError>;

impl<E> From<E> for DiagnosticError
where
    E: Diagnostic + Send + Sync,
{
    fn from(error: E) -> Self {
        let meta = ErrorMeta {
            net: error.net(),
            path: error.path(),
            parse_report: error.parse_report(),
        };

        Self {
            category: error.category(),
            meta: Some(meta),
            label: error.label(),
            advice: error.advice(),
            error: Box::new(error),
        }
    }
}

pub struct NetMeta {
    pub host: Host,
    pub url: Option<Url>,
}

pub struct PathMeta {
    pub path: PathBuf,
}

pub struct ParseMeta {
    pub input: String,
    pub row: usize,
    pub col: usize,
    pub path: Option<PathBuf>,
}

pub trait Net {
    fn net(&self) -> Option<NetMeta> {
        None
    }
}

pub trait FsPath {
    fn path(&self) -> Option<PathMeta> {
        None
    }
}

pub trait Parseable {
    fn parse_report(&self) -> Option<ParseMeta> {
        None
    }
}

pub trait Diagnostic: std::error::Error + Send + Sync + Parseable + Net + FsPath + 'static {
    fn category(&self) -> DiagnosticCategory;
    fn label(&self) -> String;
    fn advice(&self) -> Option<String>;
}

// This is needed so Box<dyn Diagnostic> is correctly treated as an Error.
impl std::error::Error for Box<dyn Diagnostic> {}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DiagnosticCategory {
    /// oro::misc
    Misc,
    /// oro::net
    Net,
    /// oro::fs
    Fs,
    /// oro::parse
    Parse,
}

pub trait AsDiagnostic<T, E> {
    fn as_diagnostic(self, subpath: impl AsRef<str>) -> std::result::Result<T, DiagnosticError>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> AsDiagnostic<T, E> for Result<T, E> {
    fn as_diagnostic(self, label: impl AsRef<str>) -> Result<T, DiagnosticError> {
        self.map_err(|e| DiagnosticError {
            category: DiagnosticCategory::Misc,
            error: Box::new(e),
            label: label.as_ref().into(),
            advice: None,
            meta: None,
        })
    }
}
