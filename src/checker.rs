use swc_core::{common::SourceFile, ecma::ast::Program};
use miette::Report;
use crate::{ast::Ast, walk_decl, walk_ty, DeclContext, TyContext};
pub struct Checker {
    pub ast: Ast,
    pub decl_context: DeclContext,
    pub ty_context: TyContext
}

impl Checker {
    pub fn new(ast: Ast, errors: &mut Vec<Report>) -> Checker{
        let mut decl_context = DeclContext::default();
        walk_decl(&mut decl_context, &ast, errors);
        let mut ty_context = TyContext::default();
        walk_ty(&mut ty_context, &decl_context, errors);
        Checker {
            ast,
            decl_context,
            ty_context
        }
    }
}