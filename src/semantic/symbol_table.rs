use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::types::Type;

/// Public interface for the symbol table
pub struct SymbolTable {
    node: Rc<RefCell<SymbolTableNode>>,
}

// struct for a symbol table
struct SymbolTableNode {
    // the parent scope
    parent: Option<Rc<RefCell<SymbolTableNode>>>,
    // the current scope
    scope: HashMap<String, (Type, bool)>,
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
    pub fn insert(&self, key: String, value: Type) {
        self.node.borrow_mut().insert(key, value, false);
    }

    /// Inserts a new symbol into the current table scope
    pub fn insert_mutable(&self, key: String, value: Type) {
        self.node.borrow_mut().insert(key, value, true);
    }

    /// Inserts a new symbol into the current table scope
    pub fn insert_custom(&self, key: String, value: Type, is_mutable: bool) {
        self.node.borrow_mut().insert(key, value, is_mutable);
    }

    /// Tests if a symbol is declared in the current or parent scopes
    pub fn test(&self, key: &String) -> bool {
        self.node.borrow_mut().test(key)
    }

    /// Gets the datatype of a symbol, if it exists
    pub fn get_type<'a>(&'a self, key: &String) -> Option<Type> {
        self.node.borrow_mut().get_type(key)
    }

    /// Gets the datatype of a symbol, if it exists, and if its mutable
    pub fn get_type_and_mut<'a>(&'a self, key: &String) -> Option<(Type, bool)> {
        self.node.borrow_mut().get_type_and_mut(key)
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
    pub fn new_from_parent(parent: &Rc<RefCell<SymbolTableNode>>) -> SymbolTableNode {
        SymbolTableNode {
            parent: Some(Rc::clone(&parent)),
            scope: HashMap::new(),
        }
    }

    /// Inserts a new symbol into the current scope
    pub fn insert(&mut self, key: String, value: Type, is_mutable: bool) {
        self.scope.insert(key, (value, is_mutable));
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

    /// Returns the symbol's datatype
    pub fn get_type<'a>(&'a mut self, key: &String) -> Option<Type> {
        // Try to get the type in the current scope
        if let Some((entry, _)) = self.scope.get(key) {
            // TODO: Change to allow other types of datatypes: functions, classes, maps
            return Some(entry.clone());
        }

        // Try to get the type in the parent scope
        match &self.parent {
            Some(parent) => {
                parent.as_ref().borrow_mut().get_type(key)
                // parent.get_type(key)
            }
            None => None,
        }
    }

    /// Returns the symbol's datatype and mutability
    pub fn get_type_and_mut<'a>(&'a mut self, key: &String) -> Option<(Type, bool)> {
        // Try to get the type in the current scope
        if let Some((entry, mutable)) = self.scope.get(key) {
            // TODO: Change to allow other types of datatypes: functions, classes, maps
            return Some((entry.clone(), *mutable));
        }

        // Try to get the type in the parent scope
        match &self.parent {
            Some(parent) => {
                parent.as_ref().borrow_mut().get_type_and_mut(key)
                // parent.get_type(key)
            }
            None => None,
        }
    }
}
