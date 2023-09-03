use inkwell::{context::Context, types::BasicTypeEnum};

pub struct Ctx<'ctx> {
    pub context: &'ctx Context,
}

impl Ctx<'_> {
    pub fn get_low_type(&self) -> BasicTypeEnum {
        self.context.i32_type().into()
    }
}
