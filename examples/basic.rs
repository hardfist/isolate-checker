use std::io::{self, Write};

use isolate_checker::{ast::Ast, InferContext, IrContext, TyContext, TypeInference};

fn main() {
    let code = r#"
    let a = 1;
    let b = 2;
    "#;

    let ast = Ast::new_from(code.into());
    dbg!(&ast);
    let mut errors = Vec::new();
    let ir_ctx = IrContext::new(ast, &mut errors);
    let mut infer = TypeInference::default();
    let infer_ctx = InferContext::new(&ir_ctx);

    for item in ir_ctx.ast.items() {
        infer.infer_item(&infer_ctx, item);
    }
    errors.extend( infer.reports);

    let report_hander = miette::GraphicalReportHandler::new();

    for err in errors {
        let err = err.with_source_code(code);
        let mut output = String::new();

        report_hander.render_report(&mut output, err.as_ref()).unwrap();

        io::stdout().write_all(output.as_bytes()).unwrap();

    }
    //let mut infer_context = Default::default();
}
