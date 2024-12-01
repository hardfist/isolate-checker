use crate::ast::Ast;
use la_arena::{Arena, Idx};
use miette::Report;
use rangemap::RangeMap;
use swc_core::common::BytePos;
use swc_core::ecma::visit::VisitWith;
use swc_core::ecma::{ast::FnDecl, visit::Visit};

#[derive(Debug)]
pub struct DeclContext {
    pub root_scope: ScopeId,
    pub scopes: Arena<Scope>,
    pub scopemap: RangeMap<BytePos, ScopeId>,
}
impl Default for DeclContext {
    fn default() -> Self {
        let mut scopes = Arena::default();
        let root = scopes.alloc(Scope::default());
        DeclContext {
            root_scope: root,
            scopes: Default::default(),
            scopemap: Default::default(),
        }
    }
}
impl DeclContext {
    pub fn alloc_scope(&mut self, current: ScopeId) -> ScopeId {
        self.scopes.alloc(Scope {
            parent: Some(current),
        })
    }
    pub fn find_scope(&self, pos: BytePos) -> Option<ScopeId> {
        self.scopemap.get(&pos).copied()
    }
}

/// walk all declaration in ast
pub fn build_scope(decl_ctx: &mut DeclContext, ast: &Ast, _errors: &mut Vec<Report>) {
    // build scope
    struct ScopeBuilder<'a> {
        decl_ctx: &'a mut DeclContext,
        current: ScopeId,
    }
    impl<'a> Visit for ScopeBuilder<'a> {
        // scope
        fn visit_fn_decl(&mut self, node: &FnDecl) {
            let current = self.decl_ctx.alloc_scope(self.current);
            let old_current = self.current;
            self.current = current;
            node.visit_children_with(self);
            self.current = old_current;
        }
    }
    ast.module.visit_with(&mut ScopeBuilder {
        current: decl_ctx.root_scope,
        decl_ctx,
    });
}



/// scope
///
#[derive(Debug, Default)]
pub struct Scope {
    pub parent: Option<ScopeId>,
}

pub type ScopeId = Idx<Scope>;
