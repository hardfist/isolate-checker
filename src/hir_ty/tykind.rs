use super::Ty;
use crate::infer::InferenceVar;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) enum TyKind {
    Number,
    String,
    Unknown,
    Error,
    Fn(Function),
    Infer(InferenceVar),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct Function {
    pub params: Ty,
    pub r#return: Ty,
}
