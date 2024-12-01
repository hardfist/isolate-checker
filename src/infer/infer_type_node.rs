use swc_core::ecma::ast::{TsKeywordTypeKind, TsTypeAnn};

use crate::hir_ty::Ty;

use super::{infer_value_node::InferCtx, TypeInference};

impl TypeInference {
    // infer type for type_node
    pub fn infer_type_node(&mut self, ctx: &InferCtx<'_>, ty_node: &TsTypeAnn) -> Ty {
        use swc_core::ecma::ast::TsType;
        match ty_node.type_ann.as_ref() {
            TsType::TsKeywordType(t) => match t.kind {
                TsKeywordTypeKind::TsNumberKeyword => ctx.hir_ctx.ty_ctx.number.clone(),
                TsKeywordTypeKind::TsStringKeyword => ctx.hir_ctx.ty_ctx.string.clone(),
                _ => ctx.hir_ctx.ty_ctx.error.clone(),
            },
            _ => {
                todo!()
            }
        }
    }
}
