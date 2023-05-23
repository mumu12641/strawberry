use std::{fmt::format, fs::File, io::Write};

use crate::{grammar::ast::class::Class, utils::table::Tables};

/// * Build constant
/// * Build class name table
/// * Build dispatch table
/// * Build class obj table   
/// * IO_protObj
/// * Emit other code
pub struct CodeGenerator<'a> {
    pub classes: Vec<Class>,
    pub tables: Tables,
    pub asm_file: &'a mut File,
}

//  String prototype
//          string ascii
//          len
//          tag     <- 2
//          object size
//          dispatch_table
//
//
impl<'a> CodeGenerator<'a> {
    pub fn code_generate(&mut self) {
        self.code_constants();
    }

    fn write(&mut self, s: String, tab: bool) {
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

    fn code_constants(&mut self) {
        self.write(".text".to_string(), true);
        self.write(".section    .data".to_string(), true);
        let mut num = 0;
        for str_ in &self.tables.string_table.clone() {
            self.write(".align 8".to_string(), true);
            self.write(format!("str_const_ascii_{}:", num), false);
            self.write(format!(".ascii \"{}\"", str_), true);
            self.write("".to_string(), false);

            self.write(".align 8".to_string(), true);
            self.write(format!("str_const_{}:", num), false);
            self.write(format!(".quad str_const_ascii_{}", num), true);
            self.write(format!(".quad {}", str_.len()), true);
            self.write(format!(".quad 2"), true);
            self.write(format!(".quad string_dispatch_table"), true);
            self.write(format!(".quad {}", 5 * 8), true);
            self.write(format!("str_const_end_{}:", num), false);
            self.write("".to_string(), false);

            num += 1;
        }

        num = 0;
        for int_ in &self.tables.int_table.clone() {
            self.write(".align 8".to_string(), true);
            self.write(format!("int_const_{}:", num), false);
            self.write(format!(".quad {}", int_), true);
            self.write(format!(".quad 1"), true);
            self.write(format!(".quad int_dispatch_table"), true);
            self.write(format!(".quad {}", 4 * 8), true);
            self.write("".to_string(), false);
            num += 1;
        }
        self.write(".align 8".to_string(), true);
        self.write(format!("int_dispatch_table:"), false);
        self.write(".align 8".to_string(), true);
        self.write(format!("string_dispatch_table:"), false);
    }
}
