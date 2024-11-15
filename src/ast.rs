use swc_core::common::sync::Lrc;
use swc_core::common::{FileName, SourceMap};
use swc_core::ecma::ast::{Module, ModuleItem, Program};
use swc_core::ecma::parser::{self, Parser, StringInput, TsConfig, TsSyntax};

#[derive(Debug)]
pub struct Ast {
    module: Module,
}

impl Ast {
    pub fn new_from(code: String) -> Ast {
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(FileName::Custom("test.ts".to_string()).into(), code);
        let mut parser = Parser::new(
            parser::Syntax::Typescript(TsSyntax::default()),
            StringInput::from(&*fm),
            None,
        );
        let ast = parser.parse_module().expect("Failed to parse module");
        Ast { module: ast }
    }
    pub fn items(&self) -> &Vec<ModuleItem> {
        &self.module.body
    }
}

#[derive(Debug,Hash,PartialEq, Eq,Clone,Copy)]
pub struct NodeId(u32);
