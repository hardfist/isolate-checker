use miette::{Diagnostic, Report, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("inference unify failed")]
pub struct UnifyReport {
    #[label]
    pub span: Option<SourceSpan>,
    #[related]
    pub chain: Vec<Report>,
}
