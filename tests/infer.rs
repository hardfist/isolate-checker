use std::path::Path;
use std::fs;
use std::sync::Arc;
use isolate_checker::ModuleChecker;
use miette::IntoDiagnostic;

#[test]
pub fn run_test() -> miette::Result<()> {
    let root = std::env!("CARGO_MANIFEST_DIR");
    let fixtures_dir = Path::new(root).join("./tests/fixtures");
    let cases = std::fs::read_dir(fixtures_dir).expect("get fixture failed");
    for case in cases.into_iter() {
        let case = case.into_diagnostic()?;
        assert!(case.path().extension().map_or(false, |x| dbg!(x).eq("ts")));
        let content = fs::read_to_string(case.path()).into_diagnostic()?;
        let checker = ModuleChecker::new(Arc::new(content));
        checker.check();
    }
    Ok(())
}