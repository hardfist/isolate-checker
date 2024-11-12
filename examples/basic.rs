use isolate_checker::{ast::Ast, Checker};


fn main() {
    let code = r#"
    let a = 1;
    let b = 2;
    "#;
    
    let ast = Ast::new_from(code.into());
    dbg!(&ast);
    let mut errors = Vec::new();
    let checker = Checker::new(ast, &mut errors);
    //let mut infer_context = Default::default();
    
}
