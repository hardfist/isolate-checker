use std::{
    io::{self, Write},
    sync::Arc,
};

use crate::{ast::Ast, InferContext, IrContext, TypeInference};
pub struct Checker {}
impl Checker {
    pub fn check(code: String) {
        let ast = Ast::new_from(code.clone());
        let mut errors = Vec::new();
        let ir_ctx = IrContext::new(ast, &mut errors);
        let mut infer = TypeInference::default();
        let infer_ctx = InferContext::new(&ir_ctx);

        for item in ir_ctx.ast.items() {
            infer.infer_item(&infer_ctx, item);
        }
        errors.append(&mut infer.reports);

        let report_handler = miette::GraphicalReportHandler::new();
        let code = Arc::new(code);
        for err in errors {
            let err = err.with_source_code(code.clone());
            let mut output = String::new();

            report_handler
                .render_report(&mut output, err.as_ref())
                .unwrap();

            io::stdout().write_all(output.as_bytes()).unwrap();
        }
    }
}
