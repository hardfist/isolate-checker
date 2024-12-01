use isolate_checker::{
    ast::Ast,
    hir_ctx::HirCtx,
    infer::{InferContext, TypeInference},
};
use miette::Result;
use std::{
    io::{self, Write},
    sync::Arc,
};

fn main() -> Result<()> {
    let code = r#"
    let a = 1;
    let b = a;
    let c = 'ss';
    let d;
    a;
    b;
    "#;

    let ast = Ast::new_from(Arc::new(code.to_string()))?;
    let mut errors = Vec::new();
    let ir_ctx = HirCtx::new(&ast, &mut errors);
    let mut infer = TypeInference::default();
    let infer_ctx = InferContext::new(&ir_ctx);

    for item in ir_ctx.ast.items() {
        infer.infer_item(&infer_ctx, item);
    }
    errors.append(&mut infer.reports);

    let report_hander = miette::GraphicalReportHandler::new();

    for err in errors {
        let err = err.with_source_code(code);
        let mut output = String::new();

        report_hander
            .render_report(&mut output, err.as_ref())
            .unwrap();

        io::stdout().write_all(output.as_bytes()).unwrap();
    }
    for (id, ty) in infer.typemap.clone() {
        let ty = infer.norm(&ty);
        dbg!(id, ty);
    }
    Ok(())
    //let mut infer_context = Default::default();
}
