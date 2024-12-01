use miette::Report;

use crate::{
    ast::Ast,
    hir::{build_scope, ScopeContext},
    hir_ty::{TyContext},
};
pub struct HirCtx<'a> {
    pub ast: &'a Ast,
    pub decl_ctx: ScopeContext,
    pub ty_ctx: TyContext,
}

impl<'a> HirCtx<'a> {
    pub fn new(ast: &'a Ast, errors: &mut Vec<Report>) -> HirCtx<'a> {
        let mut decl_context = ScopeContext::default();
        build_scope(&mut decl_context, ast, errors);
        let ty_context = TyContext::default();
        HirCtx {
            ast,
            decl_ctx: decl_context,
            ty_ctx: ty_context,
        }
    }
}
