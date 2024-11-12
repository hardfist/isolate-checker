use crate::{ast::NodeId, Ty};
use miette::Report;

#[derive(Default,Debug)]
pub struct TypeInference {
   pub typemap: Vec<(NodeId,Ty)>,
   pub reports: Vec<Report>
}

pub struct InferContext {
   
}