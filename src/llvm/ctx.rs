use inkwell::{context::Context, types::BasicTypeEnum};

use crate::{grammar::ast::Type, INT};

pub struct Ctx<'ctx> {
    pub context: &'ctx Context,
}

impl Ctx<'_> {
    pub fn get_low_type(&self, type_name: Type) -> BasicTypeEnum {
        if type_name == INT {
            return self.context.i32_type().into();
        }else{
            
        }

        unreachable!()
    }
}
