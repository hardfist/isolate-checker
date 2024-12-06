use ena::unify::{NoError, UnifyKey, UnifyValue};
use ra_ap_intern::Interned;

use crate::{
    error::UnifyFailed,
    hir_ty::{Ty, TyKind},
};

use super::{InferCtx, TypeInference};
// this is unification api
impl TypeInference {
    pub(crate) fn new_var(&mut self) -> Ty {
        let var_id = self.table.new_key(InferenceValue::Unknown);
        Interned::new(TyKind::Infer(var_id))
    }
    // FIXME: what does norm do?
    pub(crate) fn norm(&mut self, ty: &Ty) -> Ty {
        ty.clone()
    }
    pub(crate) fn unify_eq(
        &mut self,
        ctx: &InferCtx<'_>,
        x: &Ty,
        y: &Ty,
    ) -> Result<(), UnifyFailed> {
        self.unify(ctx, UnifyMode::Eq, x, y)
    }
    pub(crate) fn unify_subtype(
        &mut self,
        ctx: &InferCtx<'_>,
        x: &Ty,
        y: &Ty,
    ) -> Result<(), UnifyFailed> {
        self.unify(ctx, UnifyMode::Subtype, x, y)
    }
    pub(crate) fn unify_var_and_var(
        &mut self,
        ctx: &InferCtx<'_>,
        mode: UnifyMode,
        x: &Ty,
        y: &Ty,
    ) {

    }
    pub(crate) fn unify(
        &mut self,
        ctx: &InferCtx<'_>,
        mode: UnifyMode,
        x: &Ty,
        y: &Ty,
    ) -> Result<(), UnifyFailed> {
        let x = self.norm(x);
        let y = self.norm(y);
        match (&*x, &*y) {
            (..) if x == y => (),
            (TyKind::Infer(id1), TyKind::Infer(id2)) => {
                if id1 != id2 {
                    return Err(UnifyFailed::new(x,y, None))
                }
            },
            (TyKind::Infer(id), _) => {
                self.table.union_value(*id, InferenceValue::Known(y));
            }
            (_, TyKind::Infer(id)) => {
                self.table.union_value(*id, InferenceValue::Known(x));
            }
            _ => (),
        }
        Ok(())
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub(crate) struct InferenceVar(u32);

#[derive(Debug, Clone)]
pub(crate) enum InferenceValue {
    Unknown,
    Known(Ty),
}

impl UnifyValue for InferenceValue {
    type Error = NoError;

    fn unify_values(value1: &Self, value2: &Self) -> Result<Self, Self::Error> {
        match (value1, value2) {
            (InferenceValue::Known(_), InferenceValue::Known(_)) => {
                panic!("conflict ty")
            }
            (InferenceValue::Known(ty), InferenceValue::Unknown)
            | (InferenceValue::Unknown, InferenceValue::Known(ty)) => {
                Ok(InferenceValue::Known(ty.clone()))
            }
            (InferenceValue::Unknown, InferenceValue::Unknown) => Ok(InferenceValue::Unknown),
        }
    }
}

impl UnifyKey for InferenceVar {
    type Value = InferenceValue;

    fn index(&self) -> u32 {
        self.0
    }

    fn from_index(u: u32) -> Self {
        InferenceVar(u)
    }

    fn tag() -> &'static str {
        "type variable"
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnifyMode {
    Eq,
    Subtype,
}
