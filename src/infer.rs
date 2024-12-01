use crate::error::UnifyReport;
use crate::hir_ctx::HirCtx;
use crate::hir_ty::{Ty, TyKind};
use ena::unify::{InPlaceUnificationTable, NoError, UnifyKey, UnifyValue};
use miette::Report;
use ra_ap_intern::Interned;
use std::fmt::Debug;
use swc_core::ecma::ast::{
    Decl, Expr, ExprStmt, FnDecl, Id, Ident, Lit, ModuleItem, Pat, Stmt, TsKeywordTypeKind,
    TsTypeAnn, VarDecl, VarDeclarator,
};

#[derive(Default)]
pub struct TypeInference {
    pub typemap: Vec<(Id, Ty)>,
    pub reports: Vec<Report>,
    table: InPlaceUnificationTable<InferenceVar>,
}
impl Debug for TypeInference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeInference")
            .field("typemap", &self.typemap)
            .field("reports", &self.reports)
            .field("table", &self.table)
            .finish()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub struct InferenceVar(u32);

#[derive(Debug, Clone)]
pub enum InferenceValue {
    Unknown,
    Known(Ty),
}
impl UnifyValue for InferenceValue {
    type Error = NoError;

    fn unify_values(value1: &Self, value2: &Self) -> Result<Self, Self::Error> {
        match (value1, value2) {
            (InferenceValue::Known(_), InferenceValue::Known(_)) => {
                panic!("conflict ty")
            }
            (InferenceValue::Known(ty), InferenceValue::Unknown)
            | (InferenceValue::Unknown, InferenceValue::Known(ty)) => {
                Ok(InferenceValue::Known(ty.clone()))
            }
            (InferenceValue::Unknown, InferenceValue::Unknown) => Ok(InferenceValue::Unknown),
        }
    }
}

impl UnifyKey for InferenceVar {
    type Value = InferenceValue;

    fn index(&self) -> u32 {
        self.0
    }

    fn from_index(u: u32) -> Self {
        InferenceVar(u)
    }

    fn tag() -> &'static str {
        "type variable"
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnifyMode {
    Eq,
    Subtype,
}
// this is unification api
impl TypeInference {
    fn new_var(&mut self) -> Ty {
        let var_id = self.table.new_key(InferenceValue::Unknown);
        Interned::new(TyKind::Infer(var_id))
    }
    pub fn unify_subtype(
        &mut self,
        ctx: &InferContext<'_>,
        x: &Ty,
        y: &Ty,
    ) -> Result<(), UnifyReport> {
        self.unify(ctx, UnifyMode::Subtype, x, y)
    }
    pub fn unify(
        &mut self,
        _ctx: &InferContext<'_>,
        _mode: UnifyMode,
        x: &Ty,
        y: &Ty,
    ) -> Result<(), UnifyReport> {
        let _x = self.norm(x);
        let _y = self.norm(y);
        //  let union_var_and_val = match(infer)
        Ok(())
    }
}
// check api
impl TypeInference {
    pub fn check(&mut self, ctx: &InferContext<'_>, expr: &Expr, expected_ty: Ty) {
        let expected_ty = self.norm(&expected_ty);
        match (expected_ty.as_ref(), expr) {
            _ => {
                let got_ty = self.infer_expr(ctx, expr);
                match is_subtype(&expected_ty, &got_ty) {
                    Some(true) => (),
                    Some(false) => {}
                    None => {
                        let result = self.unify_subtype(ctx, &expected_ty, &got_ty);
                        self.report(result);
                    }
                }
            }
        }
    }
}
fn is_subtype(x: &Ty, y: &Ty) -> Option<bool> {
    match (x.as_ref(), y.as_ref()) {
        (..) if x == y => Some(true),
        _ => Some(false),
    }
}
impl TypeInference {
    // report error
    fn report(&mut self, result: Result<(), UnifyReport>) {
        if let Err(report) = result {
            self.reports.push(report.into());
        }
    }
    // infer type for type_node
    pub fn infer_type_node(&mut self, ctx: &InferContext<'_>, ty_node: &TsTypeAnn) -> Ty {
        use swc_core::ecma::ast::TsType;
        match ty_node.type_ann.as_ref() {
            TsType::TsKeywordType(t) => match t.kind {
                TsKeywordTypeKind::TsNumberKeyword => {
                    ctx.hir_ctx.ty_ctx.number.clone()
                },
                TsKeywordTypeKind::TsStringKeyword => {
                    ctx.hir_ctx.ty_ctx.string.clone()
                }
                _ => {
                    ctx.hir_ctx.ty_ctx.error.clone()
                }
            },
            _ => {
                todo!()
            }
        }
    }
    pub fn infer_module_decl(&mut self) {}
    // let a: number = 10
    pub fn infer_var_declarator(&mut self, ctx: &mut InferContext<'_>, decl: &VarDeclarator) {
        // infer ty of ty annotation
        let var_ty = if let Pat::Ident(id) = &decl.name {
            if let Some(ty_node) = id.type_ann.as_ref() {
                self.infer_type_node(ctx, ty_node.as_ref())
            } else {
                self.new_var()
            }
            
        } else {
           self.new_var()
        };

        // check anno_ty eq init_ty
        if let Some(init_expr) = &decl.init {
            self.check(ctx, init_expr, var_ty.clone());
        }
        if let Some(id) = decl.name.as_ident() {
            let name = id.sym.as_str();
            ctx.env.insert(name.to_string(), var_ty);
        }
    }
    pub fn infer_var_decl(&mut self, ctx: &mut InferContext<'_>, var_decl: &VarDecl) {
        for decl in &var_decl.decls {
            self.infer_var_declarator(ctx, decl);
        }
    }
    pub fn infer_decl(&mut self, ctx: &mut InferContext<'_>, decl: &Decl) {
        match decl {
            Decl::Fn(node) => {
                self.infer_fn_decl(ctx, node);
            }
            Decl::Var(node) => {
                self.infer_var_decl(ctx, node.as_ref());
            }
            _ => {}
        }
    }

    pub fn infer_expr_stmt(&mut self, ctx: &InferContext<'_>, expr: &ExprStmt) -> Ty {
        self.infer_expr(ctx, &expr.expr)
    }
    pub fn infer_expr(&mut self, ctx: &InferContext<'_>, expr: &Expr) -> Ty {
        match expr {
            Expr::Lit(lit) => self.infer_lit(ctx, lit),
            Expr::Ident(id) => self.infer_id(ctx, id),
            _ => {
                todo!()
            }
        }
    }
    pub fn infer_id(&mut self, ctx: &InferContext<'_>, id: &Ident) -> Ty {
        let name = id.sym.to_string();
        let ty = match ctx.env.get(&name) {
            Some(ty) => ty.clone(),
            None => ctx.hir_ctx.ty_ctx.error.clone(),
        };
        ty
    }
    pub fn infer_lit(&mut self, ctx: &InferContext<'_>, lit: &Lit) -> Ty {
        match lit {
            Lit::Num(_) => ctx.hir_ctx.ty_ctx.string.clone(),
            Lit::Str(_) => ctx.hir_ctx.ty_ctx.number.clone(),
            _ => ctx.hir_ctx.ty_ctx.unknown.clone(),
        }
    }
    pub fn infer_stmt(&mut self, ctx: &mut InferContext<'_>, stmt: &Stmt) {
        match stmt {
            Stmt::Decl(decl) => {
                self.infer_decl(ctx, decl);
            }
            Stmt::Expr(expr) => {
                self.infer_expr_stmt(ctx, expr);
            }
            _ => {
                // skip
            }
        }
    }
    pub fn infer_fn_decl(&mut self, _ctx: &InferContext<'_>, _node: &FnDecl) {}
    pub fn infer_item(&mut self, ctx: &mut InferContext<'_>, item: &ModuleItem) {
        match item {
            ModuleItem::ModuleDecl(_) => self.infer_module_decl(),
            ModuleItem::Stmt(stmt) => {
                self.infer_stmt(ctx, stmt);
            }
        }
    }
    // FIXME: what does norm do?
    pub fn norm(&mut self, ty: &Ty) -> Ty {
        ty.clone()
    }
}

pub struct InferContext<'ctx> {
    hir_ctx: &'ctx HirCtx<'ctx>,
    env: im::HashMap<String, Ty>,
}

impl<'ctx> InferContext<'ctx> {
    pub fn new(ctx: &'ctx HirCtx) -> InferContext<'ctx> {
        InferContext {
            hir_ctx: ctx,
            env: Default::default()
        }
    }
}
