use crate::{ast::NodeId, Ty};

pub struct TypeInference {
   pub typemap: Vec<(NodeId,Ty)>
}