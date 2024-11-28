use miette::Report;

use crate::{ast::Ast, hir::{walk_decl, DeclContext}, hir_ty::{walk_ty, TyContext}};
pub struct IrContext<'a> {
    pub ast: &'a Ast,
    pub decl_ctx: DeclContext,
    pub ty_ctx: TyContext,
}

impl<'a> IrContext<'a> {
    pub fn new(ast: &'a Ast, errors: &mut Vec<Report>) -> IrContext<'a> {
        let mut decl_context = DeclContext::default();
        walk_decl(&mut decl_context, &ast, errors);
        let mut ty_context = TyContext::default();
        walk_ty(&mut ty_context, &decl_context, errors);
        IrContext {
            ast,
            decl_ctx: decl_context,
            ty_ctx: ty_context,
        }
    }
}
