// use inkwell::{
//     builder::Builder, context::Context, module::Module, types::BasicType, types::BasicTypeEnum,
//     values::BasicValueEnum, AddressSpace,
// };

// use crate::{
//     grammar::ast::class::{Class, Feature},
//     utils::table::Tables,
// };
// /// * Build constant
// /// * Build class name table
// /// * Build dispatch table
// /// * Build class obj table
// /// * IO_protObj
// /// * Emit other code
// ///
// //  malloc的时候, 应该找到每一个 class 的 protObj，里
// pub struct IrGenerator<'ctx> {
//     pub classes: Vec<Class>,
//     pub context: &'ctx Context,
//     pub module: Module<'ctx>,
//     pub builder: Builder<'ctx>,
// }

// impl<'ctx> IrGenerator<'ctx> {
//     pub unsafe fn ir_generate(&self, tables: &Tables) {
//         let main_function =
//             self.module
//                 .add_function("main", self.context.i32_type().fn_type(&[], false), None);
//         let main_entry_block = self.context.append_basic_block(main_function, "entry");
//         let i32_three = self.context.i32_type().const_int(3, false);
//         self.builder.position_at_end(main_entry_block);
//         self.builder.build_return(Some(&i32_three));

//         // generate code for Int constants and String constants

//         self.code_constants(&tables);

//         // self.code_class_name_table();
//         self.code_class_prototype();

//         // Emit to file
//         let _ = self.module.print_to_file("./inkewlltest.ll");
//     }

//     unsafe fn code_constants(&self, tables: &Tables) {
//         let int_const_struct_type = self.context.struct_type(
//             &[
//                 self.context.i32_type().into(), // val
//                 // context.i64_type().ptr_type(AddressSpace::default()).into(),
//                 self.context.i32_type().into(), // tag
//                 self.context.i32_type().into(), // size
//             ],
//             false,
//         );
//         for i in &tables.int_table {
//             let temp = self.module.add_global(
//                 // int_const_struct_type as &dyn BasicType,
//                 int_const_struct_type,
//                 Some(AddressSpace::default()),
//                 "int_const",
//             );
//             let val = self
//                 .context
//                 .i32_type()
//                 .const_int(i.parse::<u32>().unwrap().into(), false);
//             temp.set_initializer(&int_const_struct_type.const_named_struct(&[
//                 val.into(),
//                 self.context.i32_type().const_int(1, false).into(),
//                 self.context.i32_type().const_int(1, false).into(),
//             ]))
//         }

//         for i in &tables.string_table {
//             let str_const_struct_type = self.context.struct_type(
//                 &[
//                     self.context
//                         .i8_type()
//                         .array_type((i.len() + 1).try_into().unwrap())
//                         .ptr_type(AddressSpace::default())
//                         .into(),
//                     self.context.i32_type().into(), // len
//                     self.context.i32_type().into(), // tag = 2
//                     self.context.i32_type().into(), // size
//                 ],
//                 false,
//             );
//             let temp = self.module.add_global(
//                 str_const_struct_type,
//                 Some(AddressSpace::default()),
//                 "string_const",
//             );
//             let global: inkwell::values::GlobalValue =
//                 self.builder.build_global_string(&i.as_str(), "str_const");
//             temp.set_initializer(
//                 &str_const_struct_type.const_named_struct(&[
//                     global.as_pointer_value().into(),
//                     self.context
//                         .i32_type()
//                         .const_int(i.len().try_into().unwrap(), false)
//                         .into(),
//                     self.context.i32_type().const_int(2, false).into(),
//                     self.context.i32_type().const_int(1, false).into(),
//                 ]),
//             )
//         }
//     }

//     unsafe fn code_class_prototype(&self) {
//         let mut tag = 3;
//         for class_ in &self.classes {
//             // class_.features
//             let int_const_struct_type = self.context.struct_type(
//                 &[
//                     self.context.i32_type().into(), // val
//                     // context.i64_type().ptr_type(AddressSpace::default()).into(),
//                     self.context.i32_type().into(), // tag
//                     self.context.i32_type().into(), // size
//                 ],
//                 false,
//             );
//             let str_const_struct_type = self.context.struct_type(
//                 &[
//                     // context.i8_type().ptr_type(AddressSpace::default()).into(), // ptr
//                     // context.,
//                     self.context
//                         .i8_type()
//                         .array_type(1)
//                         .ptr_type(AddressSpace::default())
//                         .into(),
//                     self.context.i32_type().into(), // len
//                     self.context.i32_type().into(), // tag = 2
//                     self.context.i32_type().into(), // size
//                 ],
//                 false,
//             );
//             let empty_string = self.builder.build_global_string("", "empty_const");

//             // struct type_field and vaule Vec
//             let mut type_field_vec: Vec<BasicTypeEnum> = Vec::new();
//             let mut type_field_val_vec: Vec<BasicValueEnum> = Vec::new();

//             for feature_ in &class_.features {
//                 // BasicTypeEnum
//                 if let Feature::Attribute(attr) = feature_ {
//                     // attr.type_
//                     if &attr.type_ == "String" {
//                         type_field_vec.push(BasicTypeEnum::StructType(str_const_struct_type));
//                         type_field_val_vec.push(inkwell::values::BasicValueEnum::StructValue(
//                             str_const_struct_type.const_named_struct(&[
//                                 empty_string.as_pointer_value().into(),
//                                 self.context.i32_type().const_int(0, false).into(),
//                                 self.context.i32_type().const_int(2, false).into(),
//                                 self.context.i32_type().const_int(0, false).into(),
//                             ]),
//                         ))
//                     } else if &attr.type_ == "Int" {
//                         type_field_vec
//                             .push(BasicTypeEnum::StructType(int_const_struct_type.clone()));

//                         // push a zero intager
//                         type_field_val_vec.push(inkwell::values::BasicValueEnum::StructValue(
//                             int_const_struct_type.const_named_struct(&[
//                                 self.context.i32_type().const_int(0, false).into(),
//                                 self.context.i32_type().const_int(1, false).into(),
//                                 self.context.i32_type().const_int(1, false).into(),
//                             ]),
//                         ))
//                     }
//                 }
//             }

//             type_field_vec.push(BasicTypeEnum::IntType(self.context.i32_type())); // tag
//             type_field_val_vec.push(self.context.i32_type().const_int(tag, false).into());
//             tag += 1;

//             type_field_vec.push(BasicTypeEnum::IntType(self.context.i32_type())); // size
//             type_field_val_vec.push(self.context.i32_type().const_int(1145, false).into());
//             // let a: [BasicTypeEnum; v.len()]= vec2array(v);
//             let class_type = self.context.struct_type(&type_field_vec[..], false);
//             let name = format!("{}_prototype", &class_.name);
//             let global_type_val =
//                 self.module
//                     .add_global(class_type, Some(AddressSpace::default()), &name);
//             global_type_val
//                 .set_initializer(&class_type.const_named_struct(&type_field_val_vec[..]));
//         }
//     }
// }
