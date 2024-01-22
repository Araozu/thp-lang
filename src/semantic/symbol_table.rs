use std::{collections::HashMap, rc::Rc, cell::RefCell};

// struct for a symbol table
pub struct SymbolTable<'a> {
    // the parent scope
    parent: Option<Rc<RefCell<&'a SymbolTable<'a>>>>,
    // the current scope
    scope: HashMap<String, SymbolEntry>,
}

pub enum SymbolEntry {
    // Just a Datatype
    Variable(String),
    // Contains: parameters, return type
    Function(Vec<String>, String),
}

impl SymbolTable<'_> {
    /// Creates a new, global symbol table
    pub fn new<'a>() -> SymbolTable<'a> {
        SymbolTable {
            parent: None,
            scope: HashMap::new(),
        }
    }

    /// Creates a new symbol table with a parent
    pub fn new_from_parent<'a>(parent: &'a SymbolTable<'a>) -> SymbolTable<'a> {
        SymbolTable {
            parent: Some(Rc::new(RefCell::new(parent))),
            scope: HashMap::new(),
        }
    }

    /// Inserts a new symbol into the current scope
    pub fn insert(&mut self, key: String, value: SymbolEntry) {
        self.scope.insert(key, value);
    }

    /// Tests if a symbol is declared in the current or parent scopes
    pub fn test(&self, key: &String) -> bool {
        if self.scope.contains_key(key) {
            return true;
        }

        match &self.parent {
            Some(parent) => {
                let parent = parent.borrow();
                parent.test(key)
            },
            None => false,
        }
    }
}

impl SymbolEntry {
    pub fn new_variable(datatype: String) -> SymbolEntry {
        SymbolEntry::Variable(datatype)
    }

    pub fn new_function(parameters: Vec<String>, return_type: String) -> SymbolEntry {
        SymbolEntry::Function(parameters, return_type)
    }
}
