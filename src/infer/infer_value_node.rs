use crate::hir_ctx::HirCtx;
use crate::hir_ty::Ty;
use swc_core::ecma::ast::{
    AssignExpr, AssignTarget, BindingIdent, Decl, Expr, ExprStmt, FnDecl, Ident, Lit,
    ModuleItem, Pat, SimpleAssignTarget, Stmt, VarDecl,
    VarDeclarator,
};

use super::TypeInference;

impl TypeInference {
    pub fn infer_module_decl(&mut self) {}
    // let a: number = 10
    pub fn infer_var_declarator(&mut self, ctx: &mut InferCtx<'_>, decl: &VarDeclarator) {
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
            self.check_coercion(ctx,  var_ty.clone(),&init_expr);
        }
        if let Some(id) = decl.name.as_ident() {
            let name = id.sym.as_str();
            ctx.env.insert(name.to_string(), var_ty);
        }
    }
    pub fn infer_var_decl(&mut self, ctx: &mut InferCtx<'_>, var_decl: &VarDecl) {
        for decl in &var_decl.decls {
            self.infer_var_declarator(ctx, decl);
        }
    }
    pub fn infer_decl(&mut self, ctx: &mut InferCtx<'_>, decl: &Decl) {
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

    pub fn infer_expr_stmt(&mut self, ctx: &InferCtx<'_>, expr: &ExprStmt) -> Ty {
        self.infer_expr(ctx, &expr.expr)
    }
    pub fn infer_expr(&mut self, ctx: &InferCtx<'_>, expr: &Expr) -> Ty {
        match expr {
            Expr::Lit(lit) => self.infer_lit(ctx, lit),
            Expr::Ident(id) => self.infer_id(ctx, id),
            Expr::Assign(expr) => self.infer_assign_expr(ctx, expr),
            _ => {
                dbg!(expr);
                todo!()
            }
        }
    }
    pub fn infer_binding_ident(&mut self, ctx: &InferCtx<'_>, binding_id: &BindingIdent) -> Ty {
        if let Some(anno) = &binding_id.type_ann {
            return self.infer_type_node(ctx, &anno);
        }
        self.infer_id(ctx, &binding_id.id)
    }
    pub fn infer_assign_expr(&mut self, ctx: &InferCtx<'_>, assign_expr: &AssignExpr) -> Ty {
        let left_ty = match &assign_expr.left {
            AssignTarget::Pat(_pat) => {
                todo!()
            }
            AssignTarget::Simple(simple) => match &simple {
                SimpleAssignTarget::Ident(id) => self.infer_binding_ident(ctx, id),
                _ => {
                    todo!()
                }
            },
        };
        self.check_coercion(ctx,  left_ty,&assign_expr.right)
    }
    pub fn infer_id(&mut self, ctx: &InferCtx<'_>, id: &Ident) -> Ty {
        let name = id.sym.to_string();
        let ty = match ctx.env.get(&name) {
            Some(ty) => ty.clone(),
            None => ctx.hir_ctx.ty_ctx.error.clone(),
        };
        ty
    }
    pub fn infer_lit(&mut self, ctx: &InferCtx<'_>, lit: &Lit) -> Ty {
        match lit {
            Lit::Num(_) => ctx.hir_ctx.ty_ctx.number.clone(),
            Lit::Str(_) => ctx.hir_ctx.ty_ctx.string.clone(),
            _ => ctx.hir_ctx.ty_ctx.unknown.clone(),
        }
    }
    pub fn infer_stmt(&mut self, ctx: &mut InferCtx<'_>, stmt: &Stmt) {
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
    pub fn infer_fn_decl(&mut self, _ctx: &InferCtx<'_>, _node: &FnDecl) {}
    pub fn infer_item(&mut self, ctx: &mut InferCtx<'_>, item: &ModuleItem) {
        match item {
            ModuleItem::ModuleDecl(_) => self.infer_module_decl(),
            ModuleItem::Stmt(stmt) => {
                self.infer_stmt(ctx, stmt);
            }
        }
    }
}

pub struct InferCtx<'ctx> {
    pub(crate) hir_ctx: &'ctx HirCtx<'ctx>,
    pub(crate) env: im::HashMap<String, Ty>,
}

impl<'ctx> InferCtx<'ctx> {
    pub fn new(ctx: &'ctx HirCtx) -> InferCtx<'ctx> {
        InferCtx {
            hir_ctx: ctx,
            env: Default::default(),
        }
    }
}
