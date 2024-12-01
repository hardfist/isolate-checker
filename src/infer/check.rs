use swc_core::{common::Spanned, ecma::ast::Expr};

use crate::{
    error::{ToSourceSpan, TypeMismatch},
    hir_ty::{Ty, TyKind},
};

use super::{InferCtx, TypeInference};

fn is_subtype(x: &Ty, y: &Ty) -> Option<bool> {
    match (x.as_ref(), y.as_ref()) {
        // we can't decide whether it is subtype now
        (TyKind::Infer(_), _) | (_, TyKind::Infer(_)) => None,
        (..) if x == y => Some(true),
        _ => Some(false),
    }
}
enum CheckMode {
    Coercion,
    Cast
}
// check api
impl TypeInference {
    pub fn check_coercion(
        &mut self,
        ctx: &InferCtx<'_>,
        expected_ty: Ty,
        expr: &Expr
    )-> Ty {
        self.check(CheckMode::Coercion,&ctx,  expected_ty,expr)
    }
    pub fn check_cast(
        &mut self,
        ctx: &InferCtx<'_>,
        expected_ty: Ty,
        expr: &Expr
    )-> Ty {
        self.check(CheckMode::Cast,&ctx, expected_ty,expr)
    }
    pub fn check(&mut self, mode: CheckMode,ctx: &InferCtx<'_>,  expected_ty: Ty,expr: &Expr,) -> Ty {
        let expected_ty = self.norm(&expected_ty);
        match (expected_ty.as_ref(), expr) {
            _ => {
                let got_ty = self.infer_expr(ctx, expr);
                match is_subtype(&expected_ty, &got_ty) {
                    Some(true) => (),
                    Some(false) => {
                        self.reports.push(
                            TypeMismatch {
                                span: Some(expr.span().to_source_span()),
                                expected: expected_ty.clone(),
                                got: got_ty.clone(),
                            }
                            .into(),
                        );
                    }
                    None => {
                        println!("{:?},{:?}", &expected_ty, &got_ty);
                        let result = self.unify_subtype(ctx, &expected_ty, &got_ty);
                        self.report(result);
                    }
                }
                got_ty
            }
        }
    }
}
