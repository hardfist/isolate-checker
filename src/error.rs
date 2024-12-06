use core::error;

use miette::{Diagnostic, Report, SourceSpan};
use swc_core::common::{source_map::SmallPos, Span, NO_EXPANSION};
use thiserror::Error;

use crate::hir_ty::Ty;

#[derive(Error, Debug, Diagnostic)]
#[error("inference unify failed")]
pub struct UnifyFailed {
    #[label(primary)]
    pub span: Option<SourceSpan>,
    #[related]
    pub chain: Vec<Report>,
}
#[derive(Error,Debug,Diagnostic)]
#[error("conflict known type: {left},{right}")]
pub struct UnifyConflict {
    left: String,
    right: String
}

impl UnifyFailed {
    pub fn new(expted_ty:Ty, got_ty:Ty, span: Option<Span>) -> UnifyFailed {
        let err = TypeMismatch {
            expected: expted_ty,
            got: got_ty,
            span:  span.map(|x| x.to_source_span())
        };
        return UnifyFailed {
            span: None,
            chain: vec![err.into()]
        }

    }
}


#[derive(Error, Debug, Diagnostic)]
#[error("expected {expected:?}, but got {got:?}")]
pub(crate) struct TypeMismatch {
    #[label(primary)]
    pub span: Option<SourceSpan>,
    pub expected: Ty,
    pub got: Ty,
}

pub(crate) trait ToSourceSpan {
    fn to_source_span(&self) -> SourceSpan;
}
impl ToSourceSpan for Span {
    fn to_source_span(&self) -> SourceSpan {
        let start = self.lo.to_usize();
        let end = self.hi().to_usize();

        SourceSpan::from(start..end)
    }
}
