use isolate_checker::checker::ModuleChecker;
use miette::Result;

fn main() -> Result<()> {
    let code = r#"
    let a = 1;
    let b = a;
    let c = 'ss';
    let d;
    a;
    b;
    "#;

    let mut checker = ModuleChecker::new(code.into())?;
    checker.check();
    let msg = checker.emit_error();
    println!("diagnostics:\n{:}", msg);
    Ok(())
    //let mut infer_context = Default::default();
}
