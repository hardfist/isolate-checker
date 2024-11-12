use super::Ty;

#[derive(Debug,Clone,Hash,PartialEq,Eq)] 
pub enum TyKind {
    Number,
    String,
    Fn(Function)
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Function {
    pub params: Ty,
    pub r#return: Ty
}