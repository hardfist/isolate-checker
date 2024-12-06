use super::unify::InferenceVar;
use crate::{error::UnifyFailed, hir_ty::Ty, util::FxIndexMap};
use ena::unify::InPlaceUnificationTable;
use miette::Report;
use swc_core::{common::Span, ecma::ast::Id};
#[derive(Default)]
pub(crate) struct TypeInference {
    pub(crate) typemap: FxIndexMap<Span, Ty>,
    pub(crate) reports: Vec<Report>,
    pub(crate) table: InPlaceUnificationTable<InferenceVar>,
}
impl TypeInference {
    // report error
    pub(crate) fn report(&mut self, result: Result<(), UnifyFailed>) {
        if let Err(report) = result {
            self.reports.push(report.into());
        }
    }
}
