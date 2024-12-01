use crate::infer::InferenceVar;

use super::Ty;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TyKind {
    Number,
    String,
    Unknown,
    Error,
    Fn(Function),
    Infer(InferenceVar),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Function {
    pub params: Ty,
    pub r#return: Ty,
}
