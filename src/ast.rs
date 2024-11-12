use swc_core::common::{FileName, SourceMap};
use swc_core::ecma::ast::{Module, Program};
use swc_core::ecma::parser::{self, Parser, StringInput, TsConfig, TsSyntax};
use swc_core::common::sync::Lrc;

#[derive(Debug)]
pub struct Ast {
    module: Module
}

impl Ast {
    pub fn new_from(code:String) -> Ast {
        let cm : Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(FileName::Custom("test.ts".to_string()).into(),code);
        let mut parser = Parser::new(parser::Syntax::Typescript(TsSyntax::default()),StringInput::from(&*fm),None);
        let ast = parser.parse_module().expect("Failed to parse module");
        Ast {
            module: ast
        }
    }
}

#[derive(Debug)]
pub struct NodeId(u32);