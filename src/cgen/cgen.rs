use std::{collections::HashMap, fmt::Display, fs::File, io::Write, ops::Deref};

use crate::{
    grammar::ast::{
        class::{Class, Feature},
        Type,
    },
    utils::table::{ClassTable, SymbolTable, Tables},
    BOOL, INT, OBJECT, STRING,
};

use super::ast::CodeGenerate;

#[derive(PartialEq, Eq, Clone)]
pub struct Location {
    pub reg: String,
    pub offset: i32,
    pub type_: Type,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub struct Environment {
    pub env: HashMap<String, SymbolTable<String, Location>>,
    // pub type_env: SymbolTable<String, Type>,
    pub curr_class: String,
    pub lable: usize,
}

/// * Build constant
/// * Build class name table
/// * Build dispatch table
/// * Build class obj table   
/// * IO_protObj
/// * Emit other code
/// param all save to stack
/// rax <- acc
/// save self to stack
///
pub struct CodeGenerator<'a> {
    pub classes: Vec<Class>,
    pub class_table: &'a mut ClassTable,
    pub tables: Tables,
    pub asm_file: &'a mut File,

    pub str_const_table: HashMap<String, usize>,
    pub int_const_table: HashMap<String, usize>,
    pub dispatch_table: HashMap<(String, String), usize>,
    pub environment: Environment,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(
        classes_: Vec<Class>,
        class_table_: &'a mut ClassTable,
        tables_: Tables,
        asm_file_: &'a mut File,
    ) -> CodeGenerator<'a> {
        CodeGenerator {
            classes: classes_,
            class_table: class_table_,
            tables: tables_,
            asm_file: asm_file_,

            str_const_table: HashMap::new(),
            int_const_table: HashMap::new(),
            dispatch_table: HashMap::new(),
            environment: Environment {
                env: HashMap::new(),
                // type_env: SymbolTable::new(),
                curr_class: "none".to_string(),
                lable: 0,
            },
        }
    }

    pub fn code_generate(&mut self) {
        // code for contants
        self.code_constants();

        // code for prototype
        self.code_prototype();

        // code for dispath table
        self.code_dispatch_table();

        // code for method
        self.code_method();

        // code for malloc
        self.code_malloc();

        self.code_print();

        // code for main
        self.code_main();
    }

    pub fn write(&mut self, s: String, tab: bool) {
        if tab {
            self.asm_file
                .write_all("\t".as_bytes())
                .expect("write failed");
        }
        self.asm_file.write_all(s.as_bytes()).expect("write failed");
        self.asm_file
            .write_all("\n".as_bytes())
            .expect("write failed");
    }

    pub fn method_start(&mut self) {
        self.write(format!("pushq %rbp"), true);
        self.write(format!("pushq %rbx"), true);
        self.write(format!("movq %rsp, %rbp"), true);
        self.write(format!("movq %rax, %rbx"), true);
    }
    pub fn method_end(&mut self) {
        self.write(format!("popq %rbx"), true);
        self.write(format!("popq %rbp"), true);
        self.write(format!("ret"), true);
    }

    fn code_constants(&mut self) {
        self.write(".text".to_string(), true);
        self.write("#   emit contants".to_string(), true);
        self.write(".section    .data".to_string(), true);
        let mut index = 0;
        for str_ in &self.tables.string_table.clone() {
            self.write(".align 8".to_string(), true);
            self.write(format!("str_const_ascii_{}:", index), false);
            self.write(format!(".ascii \"{}\"", str_), true);
            self.write("".to_string(), false);

            self.write(".align 8".to_string(), true);
            self.write(format!("str_const_{}:", index), false);
            self.write(format!(".quad {}", 5 * 8), true);
            self.write(format!(".quad String_dispatch_table"), true);
            self.write(format!(".quad str_const_ascii_{}", index), true);

            self.write(format!(".quad {}", str_.len() + 1), true);

            self.write("".to_string(), false);

            self.str_const_table.insert(str_.clone(), index);
            index += 1;
        }

        index = 0;
        for int_ in &self.tables.int_table.clone() {
            self.write(".align 8".to_string(), true);
            self.write(format!("int_const_{}:", index), false);
            self.write(format!(".quad {}", 3 * 8), true);
            self.write(format!(".quad Int_dispatch_table"), true);
            self.write(format!(".quad {}", int_), true);

            self.write("".to_string(), false);

            self.int_const_table.insert(int_.clone(), index);
            index += 1;
        }

        index = 0;
        for i in 0..2 {
            self.write(".align 8".to_string(), true);
            self.write(format!("bool_const_{}:", index), false);
            self.write(format!(".quad {}", 3 * 8), true);
            self.write(format!(".quad Bool_dispatch_table"), true);
            self.write(format!(".quad {}", i), true);

            self.write("".to_string(), false);
            index += 1;
        }
    }

    fn code_prototype(&mut self) {
        self.write("#   class prototype".to_string(), true);

        for class_ in &self.class_table.classes.clone() {
            let mut attr_len = 0;
            self.write(".align 8".to_string(), true);
            self.write(format!("{}_prototype:", class_.0), false);
            let inheritance = self.class_table.get_inheritance();
            for curr_class in inheritance.get(class_.0).unwrap() {
                for attr_ in &curr_class.features {
                    if let Feature::Attribute(_) = attr_ {
                        attr_len += 1;
                    }
                }
            }
            self.write(format!(".quad {}", (attr_len + 2) * 8), true);
            self.write(format!(".quad {}_dispatch_table", class_.0), true);
            for curr_class in inheritance.get(class_.0).unwrap() {
                for attr_ in &curr_class.features {
                    if let Feature::Attribute(attr) = attr_ {
                        if attr.type_ == STRING {
                            self.write(
                                format!(
                                    ".quad str_const_{}",
                                    self.str_const_table.get("").unwrap()
                                ),
                                true,
                            );
                        } else if attr.type_ == BOOL {
                            self.write(format!(".quad bool_const_0"), true);
                        } else if attr.type_ == INT {
                            self.write(
                                format!(
                                    ".quad int_const_{}",
                                    self.int_const_table.get("0").unwrap()
                                ),
                                true,
                            );
                        } else {
                            self.write(format!(".quad 0"), true);
                        }
                    }
                }
            }

            self.write(format!(""), true);
        }
    }

    fn code_dispatch_table(&mut self) {
        self.write("#   dispatch tables".to_string(), true);
        for class_ in &self.class_table.classes.clone() {
            self.write(".align 8".to_string(), true);
            self.write(format!("{}_dispatch_table:", class_.0), false);

            let inheritance = self.class_table.get_inheritance();

            let mut offset = 0;
            for curr_class in inheritance.get(class_.0).unwrap() {
                for feature_ in &curr_class.features {
                    if let Feature::Method(method_) = feature_ {
                        self.write(format!(".quad {}.{}", curr_class.name, method_.name), true);
                        self.dispatch_table
                            .insert((class_.0.to_string(), method_.name.to_string()), offset * 8);
                        offset += 1;
                    }
                }
            }
            self.write(format!(".quad {}.init", class_.0), true);
            self.write(format!(""), true);
        }
    }

    fn code_method(&mut self) {
        self.write("#   init method".to_string(), true);
        self.write(".text".to_string(), true);

        // for all classes's init method
        for class_ in &self.class_table.classes.clone() {
            self.write(format!("{}.init:", class_.0), false);
            self.method_start();

            // Object <- A <- Main
            if class_.0 != OBJECT {
                let parent = self.class_table.get_parent(class_.0);
                self.write(format!("call {}.init", parent), true);
            }

            self.environment
                .env
                .insert(class_.0.to_string(), SymbolTable::new());

            self.environment
                .env
                .get_mut(class_.0)
                .unwrap()
                .enter_scope();

            let attr_num = self.class_table.get_attr_num_recursive(class_.0);
            let mut index = 0;
            for feature in &class_.1.features {
                if let Feature::Attribute(attr) = feature {
                    let offset_ = (attr_num - class_.1.features.len() + 2 + index) * 8;
                    self.environment.env.get_mut(class_.0).unwrap().add(
                        &attr.name,
                        &Location {
                            reg: "%rbx".to_string(),
                            offset: offset_ as i32,
                            type_: attr.type_.clone(),
                        },
                    );
                    if let Some(expr_) = attr.init.deref() {
                        expr_.code_generate(self);
                    }
                    self.write(format!("movq %rax, {}(%rbx)", offset_), true);
                    index += 1;
                }
            }
            self.write(format!("movq ${}_dispatch_table, 8(%rbx)", class_.0), true);
            self.write(format!("movq %rbx, %rax"), true);
            self.method_end();
        }

        let classes = &self.classes.clone();
        for class_ in classes {
            self.environment.curr_class = class_.name.clone();

            for feature in &class_.features {
                if let Feature::Method(method) = feature {
                    self.environment
                        .env
                        .get_mut(&class_.name)
                        .unwrap()
                        .enter_scope();
                    self.environment.env.get_mut(&class_.name).unwrap().add(
                        &"self".to_string(),
                        &Location {
                            reg: "%rbp".to_string(),
                            offset: i32::MAX,
                            type_: class_.name.clone(),
                        },
                    );

                    let mut offset = 0;
                    let len = method.param.deref().len() as i32;
                    for param in method.param.deref() {
                        self.environment.env.get_mut(&class_.name).unwrap().add(
                            &param.0,
                            &Location {
                                reg: "%rbp".to_string(),
                                offset: 8 * (3 + len - 1 - offset),
                                type_: param.1.clone(),
                            },
                        );
                        offset += 1;
                    }

                    if let Some(expr_) = method.body.deref() {
                        self.environment
                            .env
                            .get_mut(&class_.name)
                            .unwrap()
                            .enter_scope();

                        self.write(format!("{}.{}:", class_.name, method.name), false);
                        self.method_start();

                        // sub rsp to store local var
                        let mut var_vec = Vec::new();
                        for expr in expr_ {
                            var_vec.append(&mut expr.get_var_num());
                        }
                        self.write(format!("subq ${}, %rsp", var_vec.len() * 8), true);
                        let mut var_index = 1;
                        for var in &var_vec {
                            self.environment.env.get_mut(&class_.name).unwrap().add(
                                &var.0,
                                &Location {
                                    reg: "%rbp".to_string(),
                                    offset: -8 * (var_index),
                                    type_: var.1.clone(),
                                },
                            );
                            var_index += 1;
                        }

                        for expr in expr_ {
                            expr.code_generate(self);
                        }

                        self.write(format!("addq ${}, %rsp", var_vec.len() * 8), true);
                        self.method_end();

                        self.environment
                            .env
                            .get_mut(&class_.name)
                            .unwrap()
                            .exit_scope();
                    }
                    self.environment
                        .env
                        .get_mut(&class_.name)
                        .unwrap()
                        .exit_scope();
                }
            }
        }
    }

    fn code_main(&mut self) {
        self.write(
            format!(
                "   .globl main
main:
    pushq $Main_prototype
    call Object.malloc
    addq $8, %rsp
    movq %rax, %rbx
    call Main.init
    movq %rbx, %rax
    call Main.main
    movq 16(%rax), %rax
    ret "
            ),
            true,
        );
    }

    fn code_malloc(&mut self) {
        self.write(format!("Object.malloc:"), false);
        self.method_start();
        self.write(
            format!(
                "movq 24(%rbp), %rax
    movq (%rax), %rdi
    call malloc"
            ),
            true,
        );

        self.method_end();
    }
    fn code_print(&mut self) {
        self.write(format!("Object.print:"), false);
        self.method_start();
        // attr is str_ascii
        self.write(format!("movq 24(%rbp), %rax"), true);
        // %rax is str_const

        self.write(format!("pushq 24(%rax)"), true);

        self.write(format!("movq 16(%rax), %rax"), true);

        self.write(format!("pushq %rax"), true);

        self.write(format!("movq $1, %rax"), true);
        self.write(format!("movq $1, %rdi"), true);
        self.write(format!("movq (%rsp), %rsi"), true);
        self.write(format!("movq 8(%rsp), %rdx"), true);
        self.write(format!("syscall"), true);

        // movq $1, %rdi
        // movq $string, %rsi
        // movq $len, %rdx
        // syscall
        self.write(format!("addq $8, %rsp"), true);
        self.write(format!("addq $8, %rsp"), true);
        self.write(format!("movq %rbx, %rax"), true);
        self.method_end();
    }
}
