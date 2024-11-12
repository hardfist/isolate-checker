use super::DeclContext;
use miette::Report;
#[derive(Debug,Default)]
pub struct TyContext {
    
}

pub fn walk_ty(ty_context:&mut TyContext,decl_context: &DeclContext, errors: &mut Vec<Report>){

}