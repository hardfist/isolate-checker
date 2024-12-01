use miette::{miette, Result};
use std::sync::Arc;
use swc_core::common::errors::{ColorConfig, Handler};
use swc_core::common::sync::Lrc;
use swc_core::common::{BytePos, FileName, LineCol, SourceMap};
use swc_core::ecma::ast::{Module, ModuleItem, Program};
use swc_core::ecma::codegen::text_writer::JsWriter;
use swc_core::ecma::codegen::{Config, Emitter};
use swc_core::ecma::parser::{self, Parser, StringInput, TsSyntax};

#[derive(Debug)]
pub struct Ast {
    pub module: Module,
}

impl Ast {
    pub fn parse(code: String) -> Program {
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(
            FileName::Custom("test.ts".to_string()).into(),
            code.to_string(),
        );
        let program = Parser::new(
            parser::Syntax::Typescript(TsSyntax::default()),
            StringInput::from(&*fm),
            None,
        )
        .parse_program()
        .unwrap();
        program
    }
    pub fn new_from(code: Arc<String>) -> Result<Ast> {
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(
            FileName::Custom("test.ts".to_string()).into(),
            code.to_string(),
        );
        let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, true, Some(cm.clone()));
        let mut parser = Parser::new(
            parser::Syntax::Typescript(TsSyntax::default()),
            StringInput::from(&*fm),
            None,
        );
        let ast = parser.parse_module().map_err(|e| {
            e.into_diagnostic(&handler).emit();
            miette!("parse failed")
        })?;
        let mut buf: Vec<u8> = vec![];
        let mut srcmap: Vec<(BytePos, LineCol)> = vec![];
        {
            let mut emitter = Emitter {
                cfg: Config::default(),
                cm: cm.clone(),
                comments: None,
                wr: JsWriter::new(cm.clone(), "\n", &mut buf, Some(&mut srcmap)),
            };
            emitter
                .emit_module(&ast)
                .map_err(|e| miette!("failed to emit module: {}", e))?;
        }
        Ok(Ast { module: ast })
    }
    pub fn items(&self) -> &Vec<ModuleItem> {
        &self.module.body
    }
}
