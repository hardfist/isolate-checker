use miette::{Diagnostic, Report, SourceSpan};
use swc_core::common::{source_map::SmallPos, Span};
use thiserror::Error;

use crate::hir_ty::Ty;

#[derive(Error, Debug, Diagnostic)]
#[error("inference unify failed")]
pub struct UnifyReport {
    #[label(primary)]
    pub span: Option<SourceSpan>,
    #[related]
    pub chain: Vec<Report>,
}

#[derive(Error, Debug, Diagnostic)]
#[error("expected {expected:?}, but got {got:?}")]
pub struct TypeMismatch {
    #[label(primary)]
    pub span: Option<SourceSpan>,
    pub expected: Ty,
    pub got: Ty
}

pub trait ToSourceSpan {
    fn to_source_span(&self) -> SourceSpan;

}
impl ToSourceSpan for Span {
    fn to_source_span(&self) -> SourceSpan {
        let start = self.lo.to_usize();
        let end = self.hi().to_usize();
        
        SourceSpan::from(start..end)
    }
}