use ra_ap_intern::Interned;


use super::TyKind;
#[derive(Debug)]
pub struct TyContext {
    pub string: Ty,
    pub number: Ty,
    pub unknown: Ty,
    pub error: Ty,
}
impl Default for TyContext {
    fn default() -> Self {
        TyContext {
            string: Interned::new(TyKind::String),
            number: Interned::new(TyKind::Number),
            unknown: Interned::new(TyKind::Unknown),
            error: Interned::new(TyKind::Error),
        }
    }
}

pub type Ty = Interned<TyKind>;

ra_ap_intern::impl_internable!(TyKind);
