use inkwell::{
    builder::Builder,
    context::{self, Context},
    module::Module,
    values::GlobalValue,
    AddressSpace,
};

use crate::{grammar::ast::class::Class, utils::table::Tables};

pub struct IrGenerator {
    pub classes: Vec<Class>,
}

impl IrGenerator {
    pub fn new(classes_: Vec<Class>) -> IrGenerator {
        IrGenerator { classes: classes_ }
    }

    pub unsafe fn ir_generate(&self, tables: &Tables, file_name: String) {
        let context = Context::create();
        let module = context.create_module(&file_name);
        let builder = context.create_builder();

        let main_function =
            module.add_function("main", context.i32_type().fn_type(&[], false), None);
        let main_entry_block = context.append_basic_block(main_function, "entry");
        let i32_three = context.i32_type().const_int(3, false);
        builder.position_at_end(main_entry_block);
        builder.build_return(Some(&i32_three));
        
        let int_const_struct_type = context.struct_type(
            &[
                context.i32_type().into(), // val
                // context.i64_type().ptr_type(AddressSpace::default()).into(),
                context.i32_type().into(), // tag
                context.i32_type().into(), // size
            ],
            false,
        );

        for i in &tables.int_table {
            let temp = module.add_global(
                int_const_struct_type,
                Some(AddressSpace::default()),
                "int_const",
            );
            let val = context
                .i32_type()
                .const_int(i.parse::<u32>().unwrap().into(), false);
            temp.set_initializer(&int_const_struct_type.const_named_struct(&[
                val.into(),
                context.i32_type().const_int(1, false).into(),
                context.i32_type().const_int(1, false).into(),
            ]))
        }

        let str_const_struct_type = context.struct_type(
            &[
                context.i8_type().ptr_type(AddressSpace::default()).into(), // ptr
                context.i32_type().into(),                                  // len
                context.i32_type().into(), // tag
                context.i32_type().into(), // size
            ],
            false,
        );

        for i in &tables.string_table {
            let temp = module.add_global(
                str_const_struct_type,
                Some(AddressSpace::default()),
                "string_const",
            );
            let global: inkwell::values::GlobalValue =
                builder.build_global_string(&i.as_str(), "str_const");
            temp.set_initializer(
                &str_const_struct_type.const_named_struct(&[
                    global.as_pointer_value().into(),
                    context
                        .i32_type()
                        .const_int(i.len().try_into().unwrap(), false)
                        .into(),
                    context.i32_type().const_int(2, false).into(),
                    context.i32_type().const_int(1, false).into(),
                ]),
            )
        }

        let _ = module.print_to_file("./inkewlltest.ll");
    }

}
