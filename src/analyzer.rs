use swc_core::ecma::visit::Visit;
use swc_core::ecma::ast::{self};

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
#[cfg(test)]
mod test {
    use swc_core::{common::{Globals, Mark, GLOBALS}, ecma::{transforms::base::resolver, visit::VisitWith}};

    use crate::ast::Ast;

    use super::Analyzer;


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
}