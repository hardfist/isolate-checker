use crate::ast::Ast;
use im::HashMap;
use la_arena::{Arena, Idx};
use miette::Report;
use rangemap::RangeMap;
use swc_core::common::{BytePos, Span, Spanned};
use swc_core::ecma::ast::{Id, Pat, TsTypeAliasDecl, VarDeclarator};
use swc_core::ecma::visit::{self, VisitWith};
use swc_core::ecma::{ast::FnDecl, visit::Visit};

#[derive(Debug)]
pub struct DeclContext {
    pub root_scope: ScopeId,
    pub scopes: Arena<Scope>,
    pub node2decl: HashMap<Span, DefId>,
    pub decls: Arena<Def>,
    pub scopemap: RangeMap<BytePos, ScopeId>,
}
impl Default for DeclContext {
    fn default() -> Self {
        let mut scopes = Arena::default();
        let root = scopes.alloc(Scope::default());
        DeclContext {
            root_scope: root,
            decls: Default::default(),
            node2decl: Default::default(),
            scopes: Default::default(),
            scopemap: Default::default(),
        }
    }
}
impl DeclContext {
    pub fn alloc_scope(&mut self, current: ScopeId, decl_id: Option<DefId>) -> ScopeId {
        self.scopes.alloc(Scope {
            decl_id,
            parent: Some(current),
            decl_map: Default::default(),
        })
    }
    pub fn find_scope(&self, pos: BytePos) -> Option<ScopeId> {
        self.scopemap.get(&pos).copied()
    }
}

/// walk all declaration in ast
pub fn walk_decl(decl_ctx: &mut DeclContext, ast: &Ast, errors: &mut Vec<Report>) {
    struct DeclVisitor<'a> {
        decl_ctx: &'a mut DeclContext,
        current: ScopeId,
    }
    impl<'a> Visit for DeclVisitor<'a> {
        // scope
        fn visit_fn_decl(&mut self, node: &FnDecl) {
            let decl_id = self.decl_ctx.decls.alloc(Def {
                scope: self.current,
                kind: DefKind::Fn(node.clone()),
            });
            let current = self.decl_ctx.alloc_scope(self.current, Some(decl_id));
            let old_current = self.current;
            self.current = current;
            node.visit_children_with(self);
            self.current = old_current;
        }
    }
    ast.module.visit_with(&mut DeclVisitor {
        current: decl_ctx.root_scope,
        decl_ctx,
    });
}

#[derive(Debug)]
pub struct Def {
    pub scope: ScopeId,
    pub kind: DefKind,
}

// FIXME: AST Clone is expensive, maybe we need to use NodeId in the future
#[derive(Debug)]
pub enum DefKind {
    Fn(FnDecl),
    TypeAlias(TsTypeAliasDecl),
}

pub type DefId = Idx<Def>;

/// scope
///
#[derive(Debug, Default)]
pub struct Scope {
    pub decl_id: Option<DefId>,
    pub parent: Option<ScopeId>,
    pub decl_map: HashMap<String, DefId>,
}

pub type ScopeId = Idx<Scope>;
