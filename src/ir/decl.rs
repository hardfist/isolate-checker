use miette::Report;

use crate::ast::Ast;

#[derive(Debug,Default)]
pub struct DeclContext {

}
/// walk all declaration in ast
pub fn walk_decl(ctx: &mut DeclContext, ast: &Ast, errors: &mut Vec<Report>){

}