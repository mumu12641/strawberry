use std::collections::HashMap;

use crate::{
    grammar::ast::{Identifier, Type},
    utils::table::SymbolTable,
};

use inkwell::{
    basic_block::BasicBlock,
    types::StructType,
    values::{BasicValueEnum, FunctionValue},
};

#[derive(Clone, Eq)]
pub enum VarEnv<'a> {
    //* class's field offset */
    Field(u32),
    //*  */
    Value(BasicValueEnum<'a>),
}

impl PartialEq for VarEnv<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Field(l0), Self::Field(r0)) => l0 == r0,
            (Self::Value(l0), Self::Value(r0)) => l0 == r0,
            _ => false,
        }
    }
}
impl VarEnv<'_> {
    pub fn into_offset(&self) -> u32 {
        if let Self::Field(off) = self {
            return *off;
        }
        panic!("error")
    }
}

pub struct Env<'a> {
    //* (class, field) -> offset */
    pub field_offset_map: HashMap<(Type, Type), u32>,

    //* (class, method) -> offset of method table */
    pub method_offset_map: HashMap<(Type, Type), usize>,

    //* for struct place holder */
    pub struct_type_place_holders: HashMap<Type, StructType<'a>>,

    //* curr class */
    pub curr_class: Type,

    pub curr_function: Option<FunctionValue<'a>>,

    pub curr_block: Option<BasicBlock<'a>>,

    //*  */
    // pub var_env: HashMap<Type, SymbolTable<Identifier, BasicValueEnum<'a>>>,
    pub var_env: HashMap<Type, SymbolTable<Identifier, VarEnv<'a>>>,
    // pub var_env: SymbolTable<Identifier, BasicValueEnum<'a>>,
}

impl Env<'_> {
    pub fn new() -> Self {
        Env {
            field_offset_map: HashMap::new(),
            method_offset_map: HashMap::new(),
            struct_type_place_holders: HashMap::new(),
            curr_class: String::from(""),
            curr_function: None,
            curr_block: None,
            // var_env: SymbolTable::new(),
            var_env: HashMap::new(),
        }
    }

    pub fn get_curr_env(&mut self) -> &SymbolTable<std::string::String, VarEnv<'_>> {
        self.var_env.get_mut(&self.curr_class).unwrap()
    }
}
