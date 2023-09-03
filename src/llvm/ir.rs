// use inkwell::{
//     builder::Builder, context::Context, module::Module, types::BasicType, types::BasicTypeEnum,
//     values::BasicValueEnum, AddressSpace,
// };

use inkwell::{
    builder::Builder,
    context::Context,
    debug_info::{DWARFEmissionKind, DWARFSourceLanguage},
    module::Module,
    types::{AnyType, BasicType, BasicTypeEnum, FloatType, IntType, StructType},
    AddressSpace,
};

use crate::{
    grammar::ast::class::{self, Class, Feature},
    utils::table::Tables,
    INT,
};

use super::ctx::{self, Ctx};
/// class prototype
/// class method table prototype
/// class init method
/// class constructor method
/// expressions
pub struct IrGenerator<'ctx> {
    pub classes: Vec<Class>,
    pub ctx: &'ctx Ctx<'ctx>,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
}

impl Class {
    pub fn get_llvm_type<'a>(&'a self, ctx: &'a Ctx) -> BasicTypeEnum {
        // ctx.i32_type().into()
        let class_prototype = ctx.context.opaque_struct_type(&self.name);
        let mut attrs: Vec<BasicTypeEnum> = vec![];
        for f in &self.features {
            if let Feature::Attribute(attr) = f {
                // attr.
                if attr.type_.clone().unwrap() == INT.to_string() {
                    // attrs.append(ctx.get_low_type());
                    attrs.push(ctx.get_low_type());
                }
            }
        }
        // let t: [BasicTypeEnum; attrs.lens()] = attrs.try_into().unwrap();
        // let t = attrs.as_slice();
        class_prototype.set_body(&attrs.as_slice(), false);
        // ctx.get_low_type()
        inkwell::types::BasicTypeEnum::StructType(class_prototype)
    }
}

impl<'ctx> IrGenerator<'ctx> {
    pub unsafe fn ir_generate(&self, tables: &Tables) {
        // self.builder.build_global_string(value, name)
        // self.module.add_function(name, ty, linkage)
        // self.module.add_global(type_, address_space, name)
        // self.context.append_basic_block

        //* generate main function */
        let main_function =
            self.module
                .add_function("main", self.ctx.get_low_type().fn_type(&[], false), None);
        // let main_function = self.module.add_function(name, ty, linkage)

        let main_entry_block = self.ctx.context.append_basic_block(main_function, "entry");
        let zero = self.ctx.context.i32_type().const_int(0, false);
        self.builder.position_at_end(main_entry_block);

        for class in &self.classes {
            class.get_llvm_type(self.ctx);
        }
        let _ = self.builder
            .build_malloc(self.module.get_struct_type("Main").unwrap(), "m");
        self.builder.build_return(Some(&zero));
        let _ = self.module.print_to_file("./test.ll");
    }

    // fn get_type(&self) -> IntType {
    //     return self.context.i32_type();
    // // }
    // let i32_type = self.context.i32_type();
    // let i16_type = self.context.i16_type();
    // let struct_type = self.context.opaque_struct_type("my_struct");
    // let a = self.module.get_struct_type("my_struct").unwrap();
    // struct_type.set_body(&[i32_type.into()], false);
    // let main_function = self.module.add_function(
    //     "main",
    //     self.context
    //         .get_struct_type("my_struct")
    //         .unwrap()
    //         .fn_type(&[], false),
    //     None,
    // );
    // let main_entry_block = self.context.append_basic_block(main_function, "entry");
    // let i32_three = self.context.i32_type().const_int(3, false);
    // let str_val = self.context.const_struct(&[i32_three.into()], false);
    // self.builder.position_at_end(main_entry_block);
    // self.builder.build_return(Some(&str_val));
    // let _ = self.module.print_to_file("./test.ll");
}
