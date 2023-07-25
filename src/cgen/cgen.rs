use std::{collections::HashMap, fmt::Display, fs::File, io::Write, ops::Deref};

use crate::{
    grammar::ast::{
        class::{Class, Feature},
        Type,
    },
    utils::table::{ClassTable, SymbolTable, Tables},
    BOOL, DISPATCH_TABLE_OFFSET, FIELD_BASIC_OFFSET, INT, NULL_TAG_OFFSET, OBJECT, PRIMSLOT,
    RUNTIME_ERR, STRING,
};

use super::ast::CodeGenerate;

#[derive(PartialEq, Eq, Clone)]
pub struct Location {
    pub reg: String,
    pub offset: i32,
    // pub type_: Type,
}

impl Display for Location {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub struct Environment {
    pub env: HashMap<String, SymbolTable<String, Location>>,
    // pub type_env: SymbolTable<String, Type>,
    pub field_map: HashMap<(Type, Type), usize>,
    pub curr_class: String,
    pub var_offset: i32,
    pub label: usize,
    pub align_stack: usize,
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
                field_map: HashMap::new(),
                // type_env: SymbolTable::new(),
                curr_class: "none".to_string(),
                var_offset: 1,
                label: 0,
                align_stack: 0,
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
        //        self.code_malloc();

        //        self.code_print();

        self.code_abort();

        //        self.code_to_string();

        //        self.code_concat();

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
        // self.write(format!("andq $-16, %rsp"), true);
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
            self.write(format!(".quad {}", 1), true);
            self.write(format!(".quad String_dispatch_table"), true);
            self.write(format!(".quad str_const_ascii_{}", index), true);

            self.write(format!(".quad {}", str_.len()), true);

            self.write("".to_string(), false);

            self.str_const_table.insert(str_.clone(), index);
            index += 1;
        }

        index = 0;
        for int_ in &self.tables.int_table.clone() {
            self.write(".align 8".to_string(), true);
            self.write(format!("int_const_{}:", index), false);
            self.write(format!(".quad {}", 4 * 8), true);
            self.write(format!(".quad 1"), true);
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
            self.write(format!(".quad {}", 4 * 8), true);
            self.write(format!(".quad 1"), true);
            self.write(format!(".quad Bool_dispatch_table"), true);
            self.write(format!(".quad {}", i), true);

            self.write("".to_string(), false);
            index += 1;
        }
    }

    fn code_prototype(&mut self) {
        self.write("#   class prototype".to_string(), true);

        for class_ in &self.class_table.classes.clone() {
            if class_.0 == &PRIMSLOT.to_string() {
                continue;
            }
            let attr_len = self.class_table.get_attr_num_recursive(class_.0);
            self.write(".align 8".to_string(), true);
            self.write(format!("{}_prototype:", class_.0), false);

            let inheritance = self.class_table.get_inheritance();

            self.write(format!(".quad {}", (attr_len + 3) * 8), true);
            // for null
            // modify dispatch table, all attr location, init
            self.write(format!(".quad {}", 0), true);
            self.write(format!(".quad {}_dispatch_table", class_.0), true);
            let mut index = 0;
            for curr_class in inheritance.get(class_.0).unwrap() {
                for attr_ in &curr_class.features {
                    if let Feature::Attribute(attr) = attr_ {
                        self.environment.field_map.insert(
                            (curr_class.name.clone(), attr.name.clone()),
                            FIELD_BASIC_OFFSET + index * 8,
                        );
                        index += 1;
                        if attr.type_.clone().unwrap() == STRING {
                            self.write(
                                format!(
                                    ".quad str_const_{}",
                                    self.str_const_table.get("").unwrap()
                                ),
                                true,
                            );
                        } else if attr.type_.clone().unwrap() == BOOL {
                            self.write(format!(".quad bool_const_0"), true);
                        } else if attr.type_.clone().unwrap() == INT {
                            self.write(
                                format!(
                                    ".quad int_const_{}",
                                    self.int_const_table.get("0").unwrap()
                                ),
                                true,
                            );
                        } else if attr.type_.clone().unwrap() == PRIMSLOT.to_string() {
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

            let inheritance = self
                .class_table
                .get_inheritance()
                .get(class_.0)
                .unwrap()
                .clone();
            let mut offset = 0;
            let mut reverse_inheritance = inheritance.clone();
            reverse_inheritance.reverse();
            let mut method_map: HashMap<Type, Vec<String>> = HashMap::new();

            for curr_class in &inheritance {
                let mut v: Vec<String> = vec![];
                for feature_ in &curr_class.features {
                    if let Feature::Method(method_) = feature_ {
                        v.insert(0, method_.name.clone());
                    }
                }
                method_map.insert(curr_class.name.clone(), v);
            }

            for curr_class in &inheritance {
                for feature_ in &curr_class.features {
                    if let Feature::Method(method_) = feature_ {
                        for c in &reverse_inheritance {
                            // find first one override the method
                            if method_map.get(&c.name).unwrap().contains(&method_.name) {
                                // find
                                self.write(format!(".quad {}.{}", c.name, method_.name), true);
                                self.dispatch_table.insert(
                                    (class_.0.to_string(), method_.name.to_string()),
                                    offset * 8,
                                );
                                break;
                            }
                        }
                        offset += 1;
                    }
                }
            }

            self.write(format!(".quad {}.init", class_.0), true);
            self.write(format!(""), true);
        }
    }

    fn code_method(&mut self) {
        let inheritance = self.class_table.get_inheritance();
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

            let mut index = 0;
            let parents = inheritance.get(class_.0).unwrap();

            for curr_class in parents {
                for feature in &curr_class.features {
                    if let Feature::Attribute(attr) = feature {
                        let offset_ = (index + 3) * 8;
                        self.environment.env.get_mut(class_.0).unwrap().add(
                            &attr.name,
                            &Location {
                                reg: "%rbx".to_string(),
                                offset: offset_ as i32,
                                // type_: attr.type_.clone().unwrap(),
                            },
                        );
                        if let Some(expr_) = attr.init.deref() {
                            expr_.code_generate(self);
                            self.write(format!("movq %rax, {}(%rbx)", offset_), true);
                        } else {
                            let type_ = attr.type_.clone().unwrap();
                            if type_ != PRIMSLOT.to_string() {
                                self.write(
                                    format!(
                                        "movq ${}_prototype, {}(%rbx)",
                                        attr.type_.clone().unwrap(),
                                        offset_
                                    ),
                                    true,
                                )
                            }
                        }
                        index += 1;
                    }
                }
            }
            self.write(
                format!(
                    "movq ${}_dispatch_table,{}(%rbx)",
                    class_.0, DISPATCH_TABLE_OFFSET
                ),
                true,
            );
            self.write(format!("movq $1, {}(%rbx)", NULL_TAG_OFFSET), true);
            self.write(format!("movq %rbx, %rax"), true);
            self.method_end();
        }

        let classes = &self.classes.clone();
        for class_ in classes {
            self.environment.curr_class = class_.name.clone();

            for feature in &class_.features {
                match feature {
                    Feature::Attribute(_) => {}
                    _ => {
                        self.code_method_constructor(&class_.name, feature);
                    }
                }
            }
        }
    }

    fn code_method_constructor(&mut self, curr_class: &String, feature: &Feature) {
        self.environment
            .env
            .get_mut(curr_class)
            .unwrap()
            .enter_scope();
        // self.environment.env.get_mut(&class_.name).unwrap().add(
        //     &"self".to_string(),
        //     &Location {
        //         reg: "%rbp".to_string(),
        //         offset: i32::MAX,
        //         // type_: class_.name.clone(),
        //     },
        // );

        let mut offset = 0;
        let len = feature.get_param_len();
        for param in feature.clone().get_param().deref() {
            self.environment.env.get_mut(curr_class).unwrap().add(
                &param.0,
                &Location {
                    reg: "%rbp".to_string(),
                    offset: 8 * (3 + len - 1 - offset),
                    // type_: param.1.clone(),
                },
            );
            offset += 1;
        }

        if let Some(expr_) = feature.get_body().deref() {
            self.environment.var_offset = 1;
            self.environment
                .env
                .get_mut(curr_class)
                .unwrap()
                .enter_scope();

            // self.write(format!("{}.{}:", curr_class, method.name), false);
            match feature {
                Feature::Method(method) => {
                    self.write(format!("{}.{}:", curr_class, method.name), false)
                }
                Feature::Constructor(_) => {
                    self.write(format!("{}.Constructor:", curr_class,), false)
                }
                _ => {}
            }
            self.method_start();

            // sub rsp to store local var
            let mut var_vec = Vec::new();
            for expr in expr_ {
                var_vec.append(&mut expr.get_var_num());
            }
            let align_stack;

            // if attr's len is odd
            if len % 2 == 0 {
                align_stack = crate::utils::util::align_to_16_bit(var_vec.len() * 8) + 8;
            } else {
                align_stack = crate::utils::util::align_to_16_bit(var_vec.len() * 8);
            }
            self.environment.align_stack = align_stack;
            self.write(format!("subq ${}, %rsp", align_stack), true);

            for expr in expr_ {
                expr.code_generate(self);
            }

            self.environment
                .env
                .get_mut(curr_class)
                .unwrap()
                .exit_scope();
            match feature {
                Feature::Constructor(_) => {
                    self.write(format!("movq %rbx, %rax"), true);
                    self.write(format!("addq ${}, %rsp", align_stack), true);
                    self.method_end();
                }
                _ => {}
            }
        } else {
            match feature {
                Feature::Method(method) => {
                    self.write(format!("{}.{}:", curr_class, method.name), false)
                }
                Feature::Constructor(_) => {
                    self.write(format!("{}.Constructor:", curr_class,), false)
                }
                _ => {}
            }
            self.method_start();
            self.write(format!("movq $Object_prototype, %rax"), true);
            self.method_end();
        }
        self.environment
            .env
            .get_mut(curr_class)
            .unwrap()
            .exit_scope();
    }

    fn code_main(&mut self) {
        self.write(
            format!(
                ".globl main
main:
    pushq $Main_prototype
    call Object.malloc
    addq $8, %rsp
    movq %rax, %rbx
    call Main.init
    # 0x....d9b8
    movq %rbx, %rax
    subq $8, %rsp
    # 0x....d9b0
    call Main.main
    movq 24(%rax), %rax
    addq $8, %rsp
    ret "
            ),
            true,
        );
    }

    // fn code_malloc(&mut self) {
    //     self.write(format!("Object.malloc:"), false);
    //     self.method_start();
    //     self.write(
    //         format!(
    //             "movq 24(%rbp), %rax
    // movq (%rax), %rdi
    // call malloc"
    //         ),
    //         true,
    //     );

    //     self.method_end();
    // }

    // fn code_print(&mut self) {
    //     self.write(format!("Object.print:"), false);
    //     self.method_start();
    //     // param is str_type
    //     // get param
    //     self.write(format!("movq 24(%rbp), %rax"), true);
    //     // %rax is str_const

    //     // push len
    //     self.write(format!("pushq 32(%rax)"), true);

    //     // get ascii
    //     self.write(format!("movq 24(%rax), %rax"), true);

    //     // push ascii
    //     self.write(format!("pushq %rax"), true);

    //     self.write(format!("movq $1, %rax"), true);
    //     self.write(format!("movq $1, %rdi"), true);
    //     self.write(format!("movq (%rsp), %rsi"), true);
    //     self.write(format!("movq 8(%rsp), %rdx"), true);
    //     self.write(format!("syscall"), true);

    //     // movq $1, %rdi
    //     // movq $string, %rsi
    //     // movq $len, %rdx
    //     // syscall
    //     self.write(format!("addq $8, %rsp"), true);
    //     self.write(format!("addq $8, %rsp"), true);
    //     self.write(format!("movq %rbx, %rax"), true);
    //     self.method_end();
    // }

    fn code_abort(&mut self) {
        self.write(format!("abort:"), false);
        self.write(format!("movq $1, %rax"), true);
        self.write(format!("movq $2, %rdi"), true);
        self.write(
            format!(
                "movq $str_const_ascii_{}, %rsi",
                self.str_const_table.get(RUNTIME_ERR).unwrap()
            ),
            true,
        );
        self.write(format!("movq ${}, %rdx", RUNTIME_ERR.len()), true);
        self.write(format!("syscall"), true);

        self.write(format!("call exit"), true);
    }

    // fn code_to_string(&mut self) {
    //     self.write(format!("Object.to_string:"), false);
    //     self.method_start();
    //     self.write(
    //         format!(
    //             "movq $str_const_{}, %rax",
    //             self.str_const_table.get("").unwrap()
    //         ),
    //         true,
    //     );
    //     self.method_end();
    //     self.write(format!("String.to_string:"), false);
    //     self.method_start();
    //     self.write(format!("movq %rbx, %rax"), true);
    //     self.method_end();
    //     self.write(format!("Int.to_string:"), false);
    //     self.method_start();
    //     // rbx is self
    //     self.write(format!("movq $32, %rdi"), true);
    //     self.write(format!("call malloc"), true);
    //     // push ascii
    //     self.write(format!("pushq %rax"), true);
    //     // rax is str_type
    //     // 1st arg
    //     self.write(format!("movq %rax, %rdi"), true);
    //     // 2 second arg
    //     self.write(
    //         format!(
    //             "movq $str_const_ascii_{}, %rsi",
    //             self.str_const_table.get("%d").unwrap()
    //         ),
    //         true,
    //     );
    //     // 3rd arg
    //     self.write(format!("movq {}(%rbx), %rdx", INT_CONST_VAL_OFFSET), true);
    //     self.write(format!("call sprintf"), true);

    //     self.write(format!("pushq $String_prototype"), true);
    //     self.write(format!("call Object.malloc"), true);
    //     self.write(format!("addq $8, %rsp"), true);
    //     self.write(format!("call String.init"), true);

    //     self.write(format!("popq %rdi"), true);
    //     self.write(
    //         format!("movq %rdi, {}(%rax)", STRING_CONST_VAL_OFFSET),
    //         true,
    //     );
    //     self.write(format!("movq $32, {}(%rax)", 32), true);
    //     self.method_end();
    // }

    // fn code_concat(&mut self) {
    //     self.write(format!("String.concat:"), false);
    //     self.method_start();
    //     // malloc str_ascii r10
    //     self.write(format!("movq $64, %rdi"), true);
    //     self.write(format!("call malloc"), true);
    //     self.write(format!("movq %rax, %r10"), true);
    //     // move malloc's rax to rdi
    //     self.write(format!("movq %r10, %rdi"), true);
    //     // concat(dest, src)
    //     // 1st arg
    //     self.write(format!("movq 32(%rbp), %rax"), true);
    //     self.write(
    //         format!("movq {}(%rax), %rsi", STRING_CONST_VAL_OFFSET),
    //         true,
    //     );
    //     self.write(format!("call strcpy"), true);
    //     // r10 is malloc (contain dest's str)
    //     self.write(format!("movq %r10, %rdi"), true);
    //     // 2nd arg
    //     self.write(format!("movq 24(%rbp), %rax"), true);
    //     self.write(
    //         format!("movq {}(%rax), %rsi", STRING_CONST_VAL_OFFSET),
    //         true,
    //     );
    //     self.write(format!("call strcat"), true);
    //     self.write(format!("pushq $String_prototype"), true);
    //     self.write(format!("call Object.malloc"), true);
    //     self.write(format!("addq $8, %rsp"), true);
    //     self.write(format!("call String.init"), true);
    //     self.write(
    //         format!("movq %r10, {}(%rax)", STRING_CONST_VAL_OFFSET),
    //         true,
    //     );
    //     self.write(format!("movq $64, {}(%rax)", 32), true);
    //     self.method_end();
    // }
}
