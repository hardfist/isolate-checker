use std::task::Context;

use im::HashMap;
use la_arena::{Arena, Idx};
use miette::Report;
use swc_core::ecma::ast::FnDecl;

use crate::ast::{Ast, NodeId};

#[derive(Debug)]
pub struct DeclContext {
    pub root_scope: ScopeId,
    pub node2decl: HashMap<NodeId, DeclId>,
    pub decls: Arena<Decl>
}
impl Default for DeclContext {
    fn default() -> Self {
        let mut scopes = Arena::default();
        let root = scopes.alloc(Scope::default());
        DeclContext {
            root_scope: root,
            decls: Default::default(),
            node2decl: Default::default(),
        }
    }
}

/// walk all declaration in ast
pub fn walk_decl(ctx: &mut DeclContext, ast: &Ast, errors: &mut Vec<Report>) {}


#[derive(Debug)]
pub struct Decl {
    pub kind: DeclKind
}

#[derive(Debug)]
pub enum DeclKind {
    Fn(FnDecl)
}

pub type DeclId = Idx<Decl>;


/// scope
/// 
#[derive(Debug,Default)]
pub struct Scope {
    pub decl_id: Option<DeclId>,
    pub parent: Option<ScopeId>,
    pub decl_map: HashMap<String, DeclId>
}


pub type ScopeId = Idx<Scope>;

