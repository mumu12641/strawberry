#[derive(Debug, Clone)]
pub enum PrimitiveType {
    Str,
    Int,
    Bool,
    Void,
}
impl Eq for PrimitiveType {}
impl PartialEq for PrimitiveType {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
