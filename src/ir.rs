mod ty;
mod decl;

pub use ty::*;
pub use decl::*;

use miette::Report;

use crate::ast::Ast;
pub struct IrContext {
    pub ast: Ast,
    pub decl_context: DeclContext,
    pub ty_context: TyContext
}

impl IrContext {
    pub fn new(ast: Ast, errors: &mut Vec<Report>) -> IrContext{
        let mut decl_context = DeclContext::default();
        walk_decl(&mut decl_context, &ast, errors);
        let mut ty_context = TyContext::default();
        walk_ty(&mut ty_context, &decl_context, errors);
        IrContext {
            ast,
            decl_context,
            ty_context
        }
    }
}