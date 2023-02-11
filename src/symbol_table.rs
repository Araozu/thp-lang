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
        let symbol_table = HashMap::<String, String>::new();

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

    pub fn check_type(&self, identifier: &str, datatype: &str) -> bool {
        self.table
            .get_key_value(&String::from(identifier))
            .and_then(|(_, value)| {
                if value == &String::from(datatype) {
                    Some(true)
                }
                else {
                    Some(false)
                }
            })
            .unwrap_or(false)
    }
    
    pub fn get_type(&self, identifier: &str) -> Option<String> {
        self.table
            .get_key_value(&String::from(identifier))
            .and_then(|(_, value)| {
                Some(String::from(value))
            })
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

    #[test]
    fn should_check_type() {
        let mut table = SymbolTable::new();
        table.add("firstNumber", _NUMBER);
        
        assert!(table.check_type("firstNumber", _NUMBER));
    }
}
