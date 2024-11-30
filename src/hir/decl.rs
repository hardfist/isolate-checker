
use crate::ast::{Ast};
use im::HashMap;
use la_arena::{Arena, Idx};
use miette::Report;
use rangemap::RangeMap;
use swc_core::common::BytePos;
use swc_core::ecma::ast::{Id, Pat, TsTypeAliasDecl, VarDeclarator};
use swc_core::ecma::visit::{self, VisitWith};
use swc_core::ecma::{ast::FnDecl, visit::Visit};

#[derive(Debug)]
pub struct DeclContext {
    pub root_scope: ScopeId,
    pub scopes: Arena<Scope>,
    pub node2decl: HashMap<Id, DeclId>,
    pub decls: Arena<Decl>,
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
    pub fn alloc_scope(&mut self, current: ScopeId, decl_id: Option<DeclId>) -> ScopeId {
        self.scopes.alloc(Scope {
            decl_id,
            parent: Some(current),
            decl_map: Default::default(),
        })
    }
    pub fn find_decl(&self, mut scope_id: ScopeId, name: &str) -> Option<DeclId> {
        loop {
            let scope = &self.scopes[scope_id];
            if let Some(decl_id) = scope.decl_map.get(name).copied() {
                return Some(decl_id);
            }
            scope_id = scope.parent?;
        }
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
        fn visit_var_declarator(&mut self, node: &visit::swc_ecma_ast::VarDeclarator) {
            match &node.name {
                Pat::Ident(id) => {
                    let name = id.sym.to_string();
                    let decl_id = self.decl_ctx.decls.alloc(Decl {
                        scope: self.current,
                        kind: DeclKind::Var(node.clone()),
                    });
                    let id = id.to_id();
                    self.decl_ctx.node2decl.insert(id, decl_id);
                }
                _ => {
                    todo!("not support yet")
                }
            }
        }
        // scope
        fn visit_fn_decl(&mut self, node: &FnDecl) {
            let decl_id = self.decl_ctx.decls.alloc(Decl {
                scope: self.current,
                kind: DeclKind::Fn(node.clone()),
            });
            //let current = self.decl_ctx.alloc_scope(current, decl_id);
        }
    }
    ast.module.visit_with(&mut DeclVisitor {
        current: decl_ctx.root_scope,
        decl_ctx,
    });
}

#[derive(Debug)]
pub struct Decl {
    pub scope: ScopeId,
    pub kind: DeclKind,
}

// FIXME: AST Clone is expensive, maybe we need to use NodeId in the future
#[derive(Debug)]
pub enum DeclKind {
    Fn(FnDecl),
    Var(VarDeclarator),
    TypeAlias(TsTypeAliasDecl),
}

pub type DeclId = Idx<Decl>;

/// scope
///
#[derive(Debug, Default)]
pub struct Scope {
    pub decl_id: Option<DeclId>,
    pub parent: Option<ScopeId>,
    pub decl_map: HashMap<String, DeclId>,
}

pub type ScopeId = Idx<Scope>;
