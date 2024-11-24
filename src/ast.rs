use std::sync::Arc;

use swc_core::common::sync::Lrc;
use swc_core::common::{FileName, SourceMap, Span, Spanned};
use swc_core::ecma::ast::{Expr, Lit, Module, ModuleItem, Program, Stmt, VarDeclarator};
use swc_core::ecma::parser::{self, Parser, StringInput, TsConfig, TsSyntax};

#[derive(Debug)]
pub struct Ast {
    pub module: Module,
}

impl Ast {
    pub fn new_from(code: Arc<String>) -> Ast {
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(
            FileName::Custom("test.ts".to_string()).into(),
            code.to_string(),
        );
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct NodeId(Span);

impl Into<NodeId> for &Stmt {
    fn into(self) -> NodeId {
        NodeId(self.span())
    }
}
impl Into<NodeId> for &Expr {
    fn into(self) -> NodeId {
        NodeId(self.span())
    }
}
impl Into<NodeId> for &VarDeclarator {
    fn into(self) -> NodeId {
        NodeId(self.span())
    }
}
impl Into<NodeId> for &Lit {
    fn into(self) -> NodeId {
        NodeId(self.span())
    }
}

impl NodeId {
    // FIXME: this is dummy impl and will have some edge case
    pub fn from_node(node: &Node) -> NodeId {
        let span = match node {
            Node::Stmt(stmt) => stmt.span(),
            Node::Expr(expr) => expr.span(),
        };
        NodeId(span)
    }
    pub fn from_stmt(stmt: &Stmt) -> NodeId {
        NodeId(stmt.span())
    }
    pub fn from_expr(expr: &Expr) -> NodeId {
        NodeId(expr.span())
    }
    pub fn from_declarator(declarator: &VarDeclarator) -> NodeId {
        NodeId(declarator.span())
    }
    pub fn from_lit(lit: &Lit) -> NodeId {
        NodeId(lit.span())
    }
}
enum Node {
    Stmt(Stmt),
    Expr(Expr),
}
