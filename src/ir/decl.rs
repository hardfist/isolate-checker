use miette::Report;
use swc_core::ecma::ast::Program;

#[derive(Debug,Default)]
pub struct DeclContext {

}
/// walk all declaration in ast
pub fn walk_decl(ctx: &mut DeclContext, ast: &Program, errors: &mut Vec<Report>){

}