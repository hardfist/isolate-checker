mod tykind;
pub use tykind::*;
use super::DeclContext;
use miette::Report;
use ra_ap_intern::Interned;
#[derive(Debug,Default)]
pub struct TyContext {
    
}


pub type Ty = Interned<TyKind>;

ra_ap_intern::impl_internable!(TyKind);


pub fn walk_ty(ty_context:&mut TyContext,decl_context: &DeclContext, errors: &mut Vec<Report>){

}