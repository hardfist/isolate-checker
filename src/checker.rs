use miette::{Report,Result};
use swc_core::ecma::codegen::{Config, Emitter};
use std::{
    cell::{LazyCell, RefCell},
    io::{self, Write},
    sync::Arc,
};

use crate::{ast::Ast, InferContext, IrContext, TypeInference};

pub struct ModuleChecker {
    errors: RefCell<Vec<Report>>,
    code: Arc<String>,
    ast: Ast,
}
impl ModuleChecker {
    pub fn new(code: Arc<String>) -> Result<ModuleChecker> {
        let ast: Ast = Ast::new_from(code.clone())?;
        Ok(Self {
            errors: RefCell::new(vec![]),
            code,
            ast,
        })
    }
}
impl ModuleChecker {
    pub fn extends_errors(&self, errors: Vec<Report>) {
        self.errors.borrow_mut().extend(errors.into_iter());
    }
    pub fn check(&self) {
        let mut errors = Vec::new();
        let ir_ctx = IrContext::new(&self.ast, &mut errors);
        let mut infer = TypeInference::default();
        let infer_ctx = InferContext::new(&ir_ctx);

        for item in ir_ctx.ast.items() {
            infer.infer_item(&infer_ctx, item);
        }

        errors.append(&mut infer.reports);
        self.extends_errors(errors);

        let error_msg = self.emit_error();
        self.emit_type(&infer);
        println!("{:?}", error_msg);
    }
    pub fn emit_type(&self, ctx: &TypeInference) {
        for (node_id, ty) in &ctx.typemap {
            dbg!(node_id, ty);
        }
    }
    pub fn emit_error(&self) -> String {
        let report_handler = miette::GraphicalReportHandler::new();
        let error_msg: Vec<String> = self
            .errors
            .take()
            .into_iter()
            .map(|err| {
                let err = err.with_source_code(self.code.clone());
                let mut output = String::new();
                report_handler
                    .render_report(&mut output, err.as_ref())
                    .unwrap();
                output
            })
            .collect();
        error_msg.join("\n")
    }
}
