use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// Public interface for the symbol table
pub struct SymbolTable {
    node: Rc<RefCell<SymbolTableNode>>,
}

// struct for a symbol table
struct SymbolTableNode {
    // the parent scope
    parent: Option<Rc<RefCell<SymbolTableNode>>>,
    // the current scope
    scope: HashMap<String, SymbolEntry>,
}

pub enum SymbolEntry {
    // Just a Datatype
    Variable(String),
    // Contains: parameters, return type
    Function(Vec<String>, String),
}

impl SymbolTable {
    /// Creates a new, global symbol table
    pub fn new() -> SymbolTable {
        SymbolTable {
            node: Rc::new(RefCell::new(SymbolTableNode::new())),
        }
    }

    pub fn new_from_parent(parent: &SymbolTable) -> SymbolTable {
        let new_table = SymbolTableNode::new_from_parent(&parent.node);

        SymbolTable {
            node: Rc::new(RefCell::new(new_table)),
        }
    }

    /// Inserts a new symbol into the current table scope
    pub fn insert(&self, key: String, value: SymbolEntry) {
        self.node.borrow_mut().insert(key, value);
    }

    /// Tests if a symbol is declared in the current or parent scopes
    pub fn test(&self, key: &String) -> bool {
        self.node.borrow_mut().test(key)
    }
}

impl SymbolTableNode {
    /// Creates a new, global symbol table
    pub fn new<'a>() -> SymbolTableNode {
        SymbolTableNode {
            parent: None,
            scope: HashMap::new(),
        }
    }

    /// Creates a new symbol table with a parent
    pub fn new_from_parent<'a>(parent: &Rc<RefCell<SymbolTableNode>>) -> SymbolTableNode {
        SymbolTableNode {
            parent: Some(Rc::clone(&parent)),
            scope: HashMap::new(),
        }
    }

    /// Inserts a new symbol into the current scope
    pub fn insert(&mut self, key: String, value: SymbolEntry) {
        self.scope.insert(key, value);
    }

    /// Tests if a symbol is declared in the current or parent scopes
    pub fn test(&mut self, key: &String) -> bool {
        if self.scope.contains_key(key) {
            return true;
        }

        match &self.parent {
            Some(parent) => {
                let mut parent = parent.as_ref().borrow_mut();
                parent.test(key)
            }
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
