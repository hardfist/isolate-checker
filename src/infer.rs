use crate::{ast::NodeId, IrContext, Ty};
use miette::Report;
use swc_core::ecma::ast::{Decl, FnDecl, ModuleItem, Stmt};

#[derive(Default, Debug)]
pub struct TypeInference {
    pub typemap: Vec<(NodeId, Ty)>,
    pub reports: Vec<Report>,
}

impl TypeInference {
    pub fn infer_module_decl(&mut self) {}
    pub fn infer_decl(&mut self, decl: &Decl) {
        match decl {
            Decl::Fn(node) => {
                self.infer_fn_decl(node);
            }
            _ => {}
        }
    }
    pub fn infer_fn_decl(&mut self, node: &FnDecl) {}
    pub fn infer_item(&mut self, ctx: &InferContext<'_>, item: &ModuleItem) {
        match item {
            ModuleItem::ModuleDecl(_) => self.infer_module_decl(),
            ModuleItem::Stmt(stmt) => {
                match stmt {
                    Stmt::Decl(decl) => {
                        self.infer_decl(decl);
                    }
                    _ => {
                        // skip
                    }
                }
            }
        }
    }
}

pub struct InferContext<'ctx> {
    ir_ctx: &'ctx IrContext,
    env: im::HashMap<String, Ty>,
    type_args: Vec<Ty>,
    block: BlockCtx,
}

impl<'ctx> InferContext<'ctx> {
    pub fn new(ctx: &'ctx IrContext) -> InferContext<'ctx> {
        InferContext {
            ir_ctx: &ctx,
            env: Default::default(),
            type_args: Default::default(),
            block: Default::default(),
        }
    }
}

#[derive(Default, Debug)]
struct BlockCtx {
    body: Option<BodyCtx>,
}

#[derive(Debug)]
struct BodyCtx {
    result_type: Ty,
}
