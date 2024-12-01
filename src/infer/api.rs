
use crate::{error::UnifyReport, hir_ty::Ty};
use ena::unify::InPlaceUnificationTable;
use miette::Report;
use swc_core::ecma::ast::Id;
use super::unify::InferenceVar;
#[derive(Default)]
pub(crate) struct TypeInference {
    pub(crate) typemap: Vec<(Id, Ty)>,
    pub(crate) reports: Vec<Report>,
    pub(crate) table: InPlaceUnificationTable<InferenceVar>,
}
impl TypeInference {
    // report error
    pub(crate)fn report(&mut self, result: Result<(), UnifyReport>) {
        if let Err(report) = result {
            self.reports.push(report.into());
        }
    }
}
