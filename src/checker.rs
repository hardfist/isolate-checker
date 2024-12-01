use miette::{Report, Result};
use std::sync::Arc;

use crate::{
    ast::Ast,
    hir_ctx::HirCtx,
    infer::{InferCtx, TypeInference},
};

pub struct ModuleChecker {
    errors: Vec<Arc<Report>>,
    code: Arc<String>,
    ast: Ast,
}
impl ModuleChecker {
    pub fn new(code: String) -> Result<ModuleChecker> {
        let code = Arc::new(code);
        let ast: Ast = Ast::new_from(code.clone())?;
        Ok(Self {
            errors: vec![],
            code,
            ast,
        })
    }
}
impl ModuleChecker {
    pub fn all_errors(&self) -> Vec<Arc<Report>> {
        self.errors.clone()
    }
    pub fn extends_errors(&mut self, errors: Vec<Arc<Report>>) {
        self.errors.extend(errors);
    }
    pub fn check(&mut self) {
        let mut errors = Vec::new();
        let hir_ctx = HirCtx::new(&self.ast, &mut errors);
        let mut infer = TypeInference::default();
        let mut infer_ctx = InferCtx::new(&hir_ctx);

        for item in hir_ctx.ast.items() {
            infer.infer_item(&mut infer_ctx, item);
        }
        errors.append(&mut infer.reports);
        let box_errors = errors
            .into_iter()
            .map(|x| Arc::new(x.with_source_code(self.code.clone())))
            .collect();
        self.extends_errors(box_errors);
    }
    pub fn emit_error(&self) -> String {
        let theme = miette::GraphicalTheme::unicode_nocolor();
        let report_handler = miette::GraphicalReportHandler::new().with_theme(theme);
        let error_msg: Vec<String> = self
            .errors
            .clone()
            .into_iter()
            .map(|err| {
                let mut output = String::new();
                report_handler
                    .render_report(&mut output, err.as_ref().as_ref())
                    .unwrap();
                output
            })
            .collect();
        error_msg.join("\n")
    }
}
