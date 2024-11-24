use std::{
    cell::RefCell, io::{self, Write}, sync::Arc
};
use miette::Report;

use crate::{ast::Ast, InferContext, IrContext, TypeInference};

pub struct ModuleChecker {
    errors: RefCell<Vec<Report>>,
    code: Arc<String>
}
impl ModuleChecker {
    pub fn new(code: Arc<String>) -> ModuleChecker{
        Self {
            errors: RefCell::new(vec![]),
            code
        }
    }
}
impl ModuleChecker {
    pub fn extends_errors(&self,errors: Vec<Report>) {
        self.errors.borrow_mut().extend(errors.into_iter());
    }
    pub fn check(&self)  {
        let ast: Ast = Ast::new_from(self.code.clone());
        let mut errors = Vec::new();
        let ir_ctx = IrContext::new(ast, &mut errors);
        let mut infer = TypeInference::default();
        let infer_ctx = InferContext::new(&ir_ctx);

        for item in ir_ctx.ast.items() {
            infer.infer_item(&infer_ctx, item);
        }
        errors.append(&mut infer.reports);
        self.extends_errors(errors);

        let error_msg = self.emit_error();
        println!("{:?}",error_msg);
    }
    pub fn emit_error(&self) -> String{
        let report_handler = miette::GraphicalReportHandler::new();
        let error_msg: Vec<String> = self.errors.take().into_iter().map(|err| {
            let err = err.with_source_code(self.code.clone());
            let mut output = String::new();
            report_handler
                .render_report(&mut output, err.as_ref())
                .unwrap();
            output
        }).collect();
        error_msg.join("\n")
        
    }
}
