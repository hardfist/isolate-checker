use ra_ap_intern::Interned;

use crate::{error::UnifyReport, hir_ty::{Ty, TyKind}};

use super::{InferCtx, InferenceValue, TypeInference, UnifyMode};

// this is unification api
impl TypeInference {
    pub(crate) fn new_var(&mut self) -> Ty {
        let var_id = self.table.new_key(InferenceValue::Unknown);
        Interned::new(TyKind::Infer(var_id))
    }
    pub(crate) fn unify_eq(
        &mut self,
        ctx: &InferCtx<'_>,
        x:&Ty,
        y: &Ty
    ){

    }
    pub(crate) fn unify_subtype(
        &mut self,
        ctx: &InferCtx<'_>,
        x: &Ty,
        y: &Ty,
    ) -> Result<(), UnifyReport> {
        self.unify(ctx, UnifyMode::Subtype, x, y)
    }
    pub(crate) fn unify(
        &mut self,
        _ctx: &InferCtx<'_>,
        _mode: UnifyMode,
        x: &Ty,
        y: &Ty,
    ) -> Result<(), UnifyReport> {
        let x = self.norm(x);
        let y = self.norm(y);
        //  let union_var_and_val = match(infer)
        Ok(())
    }
}