use crate::{utils::table::{ClassTable, Tables, self}, RUNTIME_ERR};

pub struct Context {
    pub content: String,
    pub file_name: String,
    pub tables: Tables,
    pub class_table: ClassTable,
}

// impl Context {
//     pub fn new(files: Vec<String>) -> Self {
        
//     }
//     fn init_ctx() {
//         let mut table = table::Tables::new();
//         table.string_table.insert("".to_string());
//         table.string_table.insert("Object".to_string());
//         table.string_table.insert("%s".to_string());
//         table.string_table.insert("%d".to_string());
//         table.string_table.insert(RUNTIME_ERR.to_string());
//         table.int_table.insert("0".to_string());
//         let mut class_table = ClassTable::new();
//     }
// }
