use std::fmt::DebugList;

use crate::{ast::NodeId, error::UnifyReport, IrContext, Ty};
use ena::unify::{InPlaceUnificationTable, NoError, UnifyKey, UnifyValue};
use miette::Report;
use ra_ap_intern::Interned;
use swc_core::ecma::ast::{
    Decl, Expr, ExprStmt, FnDecl, Lit, ModuleItem, Pat, SpanExt, Stmt, TsTypeAnn, VarDecl,
    VarDeclarator,
};

#[derive(Default, Debug)]
pub struct TypeInference {
    pub typemap: Vec<(NodeId, Ty)>,
    pub reports: Vec<Report>,
    table: InPlaceUnificationTable<InferenceVar>,
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
        todo!()
    }

    fn from_index(u: u32) -> Self {
        todo!()
    }

    fn tag() -> &'static str {
        todo!()
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
        Interned::new(crate::TyKind::Infer(var_id))
    }
    pub fn unify_subtype(&mut self, ctx: &InferContext<'_>, x: &Ty, y: &Ty) -> Result<(), UnifyReport> {
        self.unify(ctx, UnifyMode::Subtype, x, y)
    }
    pub fn unify(
        &mut self,
        ctx: &InferContext<'_>,
        mode: UnifyMode,
        x: &Ty,
        y: &Ty,
    ) -> Result<(), UnifyReport> {
        let x = self.norm(x);
        let y = self.norm(y);
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
                    Some(false) => {

                    },
                    None => {
                        let result = self.unify_subtype(ctx, &expected_ty, &got_ty);
                        self.report(expr.into(), result);
                    }
                }
                
            }
        }
    }
}
fn is_subtype(x:&Ty, y: &Ty) -> Option<bool> {
    match (x.as_ref(), y.as_ref()) {
        (..) if x == y => Some(true),
        _ => {
            Some(false)
        }
    }
}
impl TypeInference {
    // report error
    fn report(&mut self, node: NodeId, result: Result<(), UnifyReport>) {
        if let Err(report) = result {
            self.reports.push(report.into());
        }
    }
    // infer type for type_node
    pub fn infer_type_node(&mut self, ctx: &InferContext<'_>, ty_node: &TsTypeAnn) -> Ty {
        todo!()
    }
    pub fn infer_module_decl(&mut self) {}
    // let a: number = 10
    pub fn infer_var_declarator(&mut self, ctx: &InferContext<'_>, decl: &VarDeclarator) {
        // infer ty of ty annotation
        let anno_ty = if let Pat::Ident(id) = &decl.name {
            if let Some(ty_node) = id.type_ann.as_ref() {
                self.infer_type_node(ctx, ty_node.as_ref())
            } else {
                self.new_var()
            }
        } else {
            todo!()
        };
        // check anno_ty with init_ty
        if let Some(init_expr) = &decl.init {
            self.check(ctx, &init_expr, anno_ty);
        }
    }
    pub fn infer_var_decl(&mut self, ctx: &InferContext<'_>, var_decl: &VarDecl) {
        for decl in &var_decl.decls {
            self.infer_var_declarator(ctx, decl);
        }
    }
    pub fn infer_decl(&mut self, ctx: &InferContext<'_>, decl: &Decl) {
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

    pub fn infer_expr_stmt(&mut self, ctx: &InferContext<'_>, expr: &ExprStmt) -> Ty{
        self.infer_expr(ctx, &expr.expr)
    }
    pub fn infer_expr(&mut self, ctx: &InferContext<'_>, expr: &Expr) -> Ty{
        match expr {
            Expr::Lit(lit) => {
                self.infer_lit(ctx, lit)
            }
            _ => {
                todo!()
            }
        }
    }
    pub fn infer_lit(&mut self, ctx: &InferContext<'_>, lit: &Lit) -> Ty {
        let ty = match lit {
            Lit::Num(_) => ctx.ir_ctx.ty_ctx.string.clone(),
            Lit::Str(_) => ctx.ir_ctx.ty_ctx.number.clone(),
            _ => ctx.ir_ctx.ty_ctx.unknown.clone(),
        };

        self.typemap.push((NodeId::from_lit(lit), ty.clone()));
        ty
    }
    pub fn infer_stmt(&mut self, ctx: &InferContext<'_>, stmt: &Stmt) {
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
    pub fn infer_fn_decl(&mut self, ctx: &InferContext<'_>, _node: &FnDecl) {}
    pub fn infer_item(&mut self, ctx: &InferContext<'_>, item: &ModuleItem) {
        match item {
            ModuleItem::ModuleDecl(_) => self.infer_module_decl(),
            ModuleItem::Stmt(stmt) => {
                self.infer_stmt(ctx, stmt);
            }
        }
    }
    // FIXME: what does norm do?
    pub fn norm(&mut self, ty: &Ty) -> Ty {
        match &**ty {
            _ => ty.clone(),
        }
    }
}

pub struct InferContext<'ctx> {
    ir_ctx: &'ctx IrContext,
    env: im::HashMap<String, Ty>,
    type_args: Vec<Ty>,
    block: BlockCtx,
}

impl<'ctx> InferContext<'ctx> {
    pub fn new(ctx: &'ctx IrContext) -> InferContext<'ctx> {
        InferContext {
            ir_ctx: &ctx,
            env: Default::default(),
            type_args: Default::default(),
            block: Default::default(),
        }
    }
}

#[derive(Default, Debug)]
struct BlockCtx {
    body: Option<BodyCtx>,
}

#[derive(Debug)]
struct BodyCtx {
    result_type: Ty,
}
