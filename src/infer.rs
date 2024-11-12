use crate::{ast::NodeId, IrContext, Ty};
use miette::Report;

#[derive(Default,Debug)]
pub struct TypeInference {
   pub typemap: Vec<(NodeId,Ty)>,
   pub reports: Vec<Report>
}

pub struct InferContext<'ctx> {
   ir_ctx: &'ctx IrContext,
   env:im::HashMap<String, Ty>,
   type_args: Vec<Ty>,
   block: BlockCtx
}

impl<'ctx> InferContext<'ctx> {
   pub fn new(ctx: &'ctx IrContext) -> InferContext<'ctx> {
      InferContext {
         ir_ctx:&ctx,
         env: Default::default(),
         type_args: Default::default(),
         block: Default::default()

      }
   }
}

#[derive(Default,Debug)]
struct BlockCtx {
   body: Option<BodyCtx>
}

#[derive(Debug)]
struct BodyCtx {
   result_type: Ty
}