use isolate_checker::{ast::Ast, IrContext, TyContext, TypeInference};


fn main() {
    let code = r#"
    let a = 1;
    let b = 2;
    "#;
    
    let ast = Ast::new_from(code.into());
    dbg!(&ast);
    let mut errors = Vec::new();
    let checker = IrContext::new(ast, &mut errors);
    let mut infer = TypeInference::default();
    let infer_ctx =  TypeInference::
    //let mut infer_context = Default::default();
    
}
