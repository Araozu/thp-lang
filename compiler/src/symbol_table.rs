use std::collections::HashMap;

use crate::semantic::Datatype;

// Primitive datatypes
pub const _NUMBER: &str = "Num";
pub const _STRING: &str = "Str";
pub const _BOOLEAN: &str = "Bool";

pub struct SymbolTable {
    /// For now just stores identifiers and datatypes
    table: HashMap<String, Datatype>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let symbol_table = HashMap::<String, Datatype>::new();

        SymbolTable {
            table: symbol_table,
        }
    }

    pub fn insert(&mut self, identifier: &str, datatype: Datatype) {
        self.table
            .insert(String::from(identifier), datatype);
    }

    pub fn has_id(&self, identifier: &String) -> bool {
        return self.table.contains_key::<String>(identifier);
    }

    pub fn check_type(&self, identifier: &String, datatype: Datatype) -> bool {
        self.table
            .get(identifier)
            .and_then(|value| {
                if *value == datatype {
                    Some(true)
                } else {
                    Some(false)
                }
            })
            .unwrap_or(false)
    }

    /// Returns the Datatype of a given identifier
    pub fn get_type(&self, identifier: &String) -> Option<&Datatype> {
        self.table
            .get(identifier)
            .and_then(|value| Some(value))
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
        table.insert("identifier", Datatype::num());

        let s = String::from("identifier");
        assert_eq!(true, table.has_id(&s))
    }

    #[test]
    fn should_check_type() {
        let mut table = SymbolTable::new();
        table.insert("firstNumber", Datatype::num());

        let s = String::from("firstNumber");
        assert!(table.check_type(&s, Datatype::num()));
    }
}
