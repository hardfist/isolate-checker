use isolate_checker::checker::ModuleChecker;
use miette::IntoDiagnostic;
use std::fs;
use std::path::Path;
use std::sync::Arc;

#[test]
pub fn run_test() -> miette::Result<()> {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let fixtures_dir = Path::new(root).join("./tests/fixtures");
    let cases = std::fs::read_dir(fixtures_dir).expect("get fixture failed");
    for case in cases.into_iter() {
        let case = case.into_diagnostic()?;
        assert!(case.path().extension().map_or(false, |x| dbg!(x).eq("ts")));
        let content = fs::read_to_string(case.path()).into_diagnostic()?;
        let mut checker = ModuleChecker::new(Arc::new(content))?;
        checker.check();
        let error_msg = checker.emit_error();
        insta::assert_snapshot!("basic.err", error_msg);
        
    }
    Ok(())
}
