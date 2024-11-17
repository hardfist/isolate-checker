mod tykind;
use super::DeclContext;
use ena::unify::{UnifyKey, UnifyValue};
use miette::Report;
use ra_ap_intern::Interned;
use swc_core::ecma::ast::CondExpr;
pub use tykind::*;
#[derive(Debug)]
pub struct TyContext {
    pub string: Ty,
    pub number: Ty,
    pub unknown: Ty,
}
impl Default for TyContext {
    fn default() -> Self {
        TyContext {
            string: Interned::new(TyKind::String),
            number: Interned::new(TyKind::Number),
            unknown: Interned::new(TyKind::Unknown),
        }
    }
}

pub type Ty = Interned<TyKind>;

ra_ap_intern::impl_internable!(TyKind);

pub fn walk_ty(ty_context: &mut TyContext, decl_ctx: &DeclContext, errors: &mut Vec<Report>) {
    for (decl_id, decl) in decl_ctx.decls.iter() {}
}
