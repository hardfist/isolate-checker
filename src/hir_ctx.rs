use miette::Report;

use crate::{
    ast::Ast,
    hir_def::{build_scope, DefCtx},
    hir_ty::TyContext,
};
pub(crate) struct HirCtx<'a> {
    pub ast: &'a Ast,
    pub ty_ctx: TyContext,
}

impl<'a> HirCtx<'a> {
    pub fn new(ast: &'a Ast, errors: &mut Vec<Report>) -> HirCtx<'a> {
        let mut def_context = DefCtx::default();
        build_scope(&mut def_context, ast, errors);
        let ty_context = TyContext::default();
        HirCtx {
            ast,
            ty_ctx: ty_context,
        }
    }
}
