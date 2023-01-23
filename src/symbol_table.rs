use std::collections::HashMap;

// Primitive datatypes
pub const _NUMBER: &str = "Num";
pub const _STRING: &str = "Str";
pub const _BOOLEAN: &str = "Bool";

pub struct SymbolTable {
    table: HashMap<String, String>
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut symbol_table = HashMap::<String, String>::new();

        SymbolTable {
            table: symbol_table,
        }
    }

    pub fn add(&mut self, identifier: &str, datatype: &str) {
        self.table.insert(String::from(identifier), String::from(datatype));
    }

    pub fn test(&self, identifier: &str) -> bool {
        return self.table.contains_key::<String>(&String::from(identifier));
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create() {
        let mut _table = SymbolTable::new();
    }

    #[test]
    fn should_add_identifier() {
        let mut table = SymbolTable::new();
        table.add("identifier", _NUMBER);
        assert_eq!(true, table.test("identifier"))
    }
}
