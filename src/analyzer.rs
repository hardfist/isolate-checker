use petgraph::graph::DiGraph;
use swc_core::{common::{ Mark}, ecma::{ast::Id, parser::parse_file_as_module, transforms::base::resolver, visit::{Visit, VisitWith}}};
use swc_core::ecma::ast::{self, Decl};
use swc_core::common::{GLOBALS,Globals};
use crate::{ast::Ast};

struct Analyzer {
    
}
impl Analyzer {
    
}
impl Visit for Analyzer {
    fn visit_ident(&mut self, node: &ast::Ident) {
        dbg!(node);
        dbg!(node.to_id());
        
    }
}

#[test]
fn test_analyzer(){
    let mut analyzer = Analyzer {

    };

    
    GLOBALS.set(&Globals::new(),|| {
            let mut program = Ast::parse(r#"
     let a = 5;
     let b = 20;
{
    let a = 3;
}
    "#.into());
        let resolver = resolver(Mark::fresh(Mark::root()), Mark::fresh(Mark::root()), false);
        program = program.apply(resolver);
        program.visit_with(&mut analyzer);
    });
    

}