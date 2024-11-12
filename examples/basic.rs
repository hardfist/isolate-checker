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
        infer
    }
    //let mut infer_context = Default::default();
}
