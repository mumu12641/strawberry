use crate::{
    parser::ast::class::Class,
    utils::table::{self, ClassTable, Tables},
    RUNTIME_ERR,
};

#[derive(Debug, Clone)]
pub struct CompileContext {
    pub classes: Vec<Class>,
    pub content: String,
    pub file_name: String,
    pub tables: Tables,
    pub class_table: ClassTable,
}

impl CompileContext {
    pub fn new() -> Self {
        let mut tables = table::Tables::new();
        tables.string_table.push("".to_string());
        tables.string_table.push("Object".to_string());
        tables.string_table.push("%s".to_string());
        tables.string_table.push("%d".to_string());
        tables.string_table.push(RUNTIME_ERR.to_string());
        tables.int_table.insert("0".to_string());
        let mut class_table = ClassTable::new();
        CompileContext {
            classes: vec![],
            content: "".to_string(),
            file_name: "".to_string(),
            tables,
            class_table,
        }
    }

    pub fn preprocess(&mut self, raw: String) {
        let ref this = raw;
        let mut result = String::new();
        let mut last_end = 0;
        for (start, part) in this.match_indices("\t") {
            result.push_str(unsafe { this.get_unchecked(last_end..start) });
            result.push_str("    ");
            last_end = start + part.len();
        }
        result.push_str(unsafe { this.get_unchecked(last_end..this.len()) });
        self.content = result;
    }
}
