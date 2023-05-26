use std::{collections::HashMap, fs::File, io::Write};

use crate::{
    grammar::ast::class::Feature,
    utils::table::{ClassTable, Tables},
    BOOL, INT, OBJECT, STRING,
};

use super::ast::CodeGenerate;

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
    // pub classes: Vec<Class>,
    pub class_table: &'a mut ClassTable,
    pub tables: Tables,
    pub asm_file: &'a mut File,

    pub str_const_table: HashMap<String, usize>,
    pub int_const_table: HashMap<String, usize>,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(
        class_table_: &'a mut ClassTable,
        tables_: Tables,
        asm_file_: &'a mut File,
    ) -> CodeGenerator<'a> {
        CodeGenerator {
            class_table: class_table_,
            tables: tables_,
            asm_file: asm_file_,

            str_const_table: HashMap::new(),
            int_const_table: HashMap::new(),
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

    fn method_start(&mut self) {
        self.write(format!("pushq %rbp"), true);
        self.write(format!("pushq %rbx"), true);
        self.write(format!("movq %rsp, %rbp"), true);
        self.write(format!("movq %rax, %rbx"), true);
    }
    fn method_end(&mut self) {
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
            self.write(format!(".quad {}", str_.len()), true);
            // self.write(format!(".quad 2"), true);

            self.write("".to_string(), false);

            self.str_const_table.insert(str_.clone(), index);
            index += 1;
        }

        index = 0;
        for int_ in &self.tables.int_table.clone() {
            self.write(".align 8".to_string(), true);
            self.write(format!("int_const_{}:", index), false);
            self.write(format!(".quad Int_dispatch_table"), true);
            self.write(format!(".quad {}", 4 * 8), true);
            self.write(format!(".quad {}", int_), true);
            // self.write(format!(".quad 1"), true);

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
            println!("{}", class_.0);
            for curr_class in inheritance.get(class_.0).unwrap() {
                println!("current class is {}", curr_class.name);
                for feature_ in &curr_class.features {
                    if let Feature::Method(method_) = feature_ {
                        self.write(format!(".quad {}.{}", curr_class.name, method_.name), true);
                    }
                }
            }
            println!();
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
            // pushq rbp
            // pushq rbx (self)
            // rbx <- rax
            // self.write(format!("pushq %rbp"), true);
            // self.write(format!("pushq %rbx"), true);
            // self.write(format!("movq %rsp %rbp"), true);
            // self.write(format!("movq %rax %rbx"), true);
            self.method_start();

            // Object <- A <- Main
            if class_.0 != OBJECT {
                let parent = self.class_table.get_parent(class_.0);
                self.write(format!("jmp {}.init", parent), true);
            }

            let attr_num = self.class_table.get_attr_num_recursive(class_.0);
            let mut index = 0;
            for feature in &class_.1.features {
                if let Feature::Attribute(attr) = feature {
                    if let Some(expr_) = *(attr.init.clone()) {
                        expr_.code_generate(self);
                    }

                    self.write(
                        format!(
                            "movq %rax, {}(%rbx)",
                            (attr_num - class_.1.features.len() + 2 + index) * 8
                        ),
                        true,
                    );
                    index += 1;
                }
            }
            // self.write(format!("pop %rbx"), true);
            // self.write(format!("pop %rbp"), true);
            // self.write(format!("ret"), true);
            self.method_end();
        }
    }
    fn code_main(&mut self) {
        self.write(format!("main:"), false);
        self.method_start();
        self.write(format!("pushq $Main_prototype"), true);
        self.write(format!("call Object.malloc"), true);

        self.write(format!("addq $8, %rsp"), true);
        self.write(format!("call Main.init"), true);
        self.method_end();
    }

    fn code_malloc(&mut self) {
        self.write(format!("Object.malloc:"), false);
        self.method_start();
        self.write(format!("movq 16(%rbp), %rax"), true);
        self.write(format!("movq (%rax), %rdi"), true);
        self.write(format!("call malloc"), true);

        self.method_end();
    }
}
