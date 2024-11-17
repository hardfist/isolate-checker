mod decl;
mod ty;

pub use decl::*;
pub use ty::*;

use miette::Report;

use crate::ast::Ast;
pub struct IrContext {
    pub ast: Ast,
    pub decl_ctx: DeclContext,
    pub ty_ctx: TyContext,
}

impl IrContext {
    pub fn new(ast: Ast, errors: &mut Vec<Report>) -> IrContext {
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
