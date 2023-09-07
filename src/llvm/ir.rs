use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType, StructType},
    AddressSpace,
};

use crate::{
    grammar::ast::{
        class::{self, Class, Feature},
        Type,
    },
    utils::table::Tables,
};
#[derive(Debug)]
/// class prototype
/// class method table prototype
/// class init method
/// class constructor method
/// expressions
pub struct IrGenerator<'ctx> {
    pub classes: Vec<Class>,
    pub ctx: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
}

impl Class {
    pub fn emit_llvm_type<'a>(&'a self, ir_genrator: &'a IrGenerator) -> BasicTypeEnum {
        //* class prototype */
        //*     NULL flag
        //*     _dispatch_table
        //*     attrs
        let class_prototype = ir_genrator.ctx.opaque_struct_type(&self.name);
        let method_prototype: BasicTypeEnum<'_> = self.emit_method_table_llvm_type(ir_genrator);

        let mut attrs: Vec<BasicTypeEnum> = vec![
            ir_genrator.get_primitive_llvm_type(),
            BasicTypeEnum::PointerType(method_prototype.ptr_type(AddressSpace::default())),
        ];

        for f in &self.features {
            match f {
                Feature::Attribute(attr) => {
                    attrs.push(ir_genrator.get_llvm_type(attr.type_.clone().unwrap()))
                }
                _ => {}
            }
        }
        class_prototype.set_body(attrs.as_slice(), false);
        BasicTypeEnum::StructType(class_prototype)
    }
    pub fn emit_method_table_llvm_type<'a>(
        &'a self,
        ir_genrator: &'a IrGenerator,
    ) -> BasicTypeEnum {
        let method_prototype = ir_genrator
            .ctx
            .opaque_struct_type(&format!("{}_dispatch_table_prototype", &self.name));
        let mut methods: Vec<BasicTypeEnum> = vec![];
        for f in &self.features {
            if let Feature::Method(method) = f {
                let mut params_type: Vec<BasicMetadataTypeEnum> =
                    vec![BasicMetadataTypeEnum::StructType(
                        ir_genrator.module.get_struct_type(&self.name).unwrap(),
                    )];
                for param in method.param.as_ref() {
                    params_type.push(ir_genrator.get_llvm_type(param.1.clone()).into())
                }
                methods.push(BasicTypeEnum::PointerType(
                    ir_genrator
                        .ctx
                        .void_type()
                        .fn_type(params_type.as_slice(), false)
                        .ptr_type(AddressSpace::default()),
                ));
            }
        }
        method_prototype.set_body(methods.as_slice(), false);
        let g = ir_genrator.module.add_global(
            method_prototype,
            Some(AddressSpace::default()),
            &format!("{}_dispatch_table", &self.name),
        );
        // g.set_initializer(method_prototype.const_named_struct(""));

        BasicTypeEnum::StructType(method_prototype)
    }
}

impl<'ctx> IrGenerator<'ctx> {
    pub fn ir_generate(&self) {
        // self.builder.build_global_string(value, name)
        // self.module.add_function(name, ty, linkage)
        // self.module.add_global(type_, address_space, name)
        // self.context.append_basic_block

        //* generate class prototypes */
        for class in &self.classes {
            class.emit_llvm_type(self);
        }

        //* generate method ir */
        //* first is init_method*/
        self.gen_init_method();
        // for class in &self.classes{

        // }

        //* generate main function */
        let main_function =
            self.module
                .add_function("main", self.ctx.i32_type().fn_type(&[], false), None);
        let main_entry_block = self.ctx.append_basic_block(main_function, "entry");
        let zero = self.ctx.i32_type().const_int(0, false);
        self.builder.position_at_end(main_entry_block);
        let _ = self
            .builder
            .build_malloc(self.module.get_struct_type("Main").unwrap(), "m");
        self.builder.build_return(Some(&zero));
        let _ = self.module.print_to_file("./test.ll");
    }

    fn gen_init_method(&self) {
        for class in &self.classes {
            let init_method = self.module.add_function(
                &format!("{}.init", &class.name),
                self.get_funtion_type(&[self.get_llvm_type(class.name.clone()).into()], None),
                None,
            );
        }
    }

    pub fn get_llvm_type(&self, type_name: Type) -> BasicTypeEnum<'ctx> {
        // is primitive
        if let Some(_) = self.get_class_llvm_type(type_name.clone()) {
            return BasicTypeEnum::StructType(self.module.get_struct_type(&type_name).unwrap());
        }

        return self.get_primitive_llvm_type();
    }

    fn get_funtion_type(
        &self,
        params: &[BasicMetadataTypeEnum<'ctx>],
        return_type: Option<BasicMetadataTypeEnum<'ctx>>,
    ) -> FunctionType<'ctx> {
        // unreachable!()
        match return_type {
            Some(return_type_) => match return_type_ {
                BasicMetadataTypeEnum::IntType(type_) => type_.fn_type(params, false),
                _ => unreachable!(),
            },
            None => self.ctx.void_type().fn_type(params, false),
        }
    }

    pub fn get_primitive_llvm_type(&self) -> BasicTypeEnum<'ctx> {
        return BasicTypeEnum::IntType(self.ctx.i32_type());
    }
    pub fn get_class_llvm_type(&self, class_name: Type) -> Option<StructType> {
        // inkwell::types::BasicTypeEnum::StructType(self.module.get_struct_type(&class_name).unwrap())
        self.module.get_struct_type(&class_name)
    }
    // pub fn get_method_llvm_type(&self)
}
