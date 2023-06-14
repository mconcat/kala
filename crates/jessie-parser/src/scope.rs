use std::cell::{RefCell, OnceCell};

use jessie_ast::{Expr, VariableCell, Declaration, Pattern, PropertyAccess, PropParam, Variable, Function, DeclarationIndex, VariablePointer};
use utils::Trie;

#[derive(Debug, PartialEq)]
pub struct LexicalScope<'a> {
    pub declarations: &'a mut Vec<Declaration>,
    pub variable_trie: Option<Box<Trie<VariablePointer>>>,
}

impl<'a> LexicalScope<'a> {
    pub fn new(declarations: &'a mut Vec<Declaration>) -> Self {
        Self {
            declarations,
            variable_trie: None,
        }
    }

    fn get_declarations(&mut self) -> &mut Vec<Declaration> {
        self.declarations
    }

    fn get_variable_trie(&mut self) -> &mut Trie<VariablePointer> {
        if self.variable_trie.is_none() {
            self.variable_trie = Some(Box::new(Trie::empty()));
        }
        self.variable_trie.as_mut().unwrap()
    }

    fn next_declaration_index(&mut self) -> DeclarationIndex {
        DeclarationIndex(self.get_declarations().len())
    }

    fn declare(&mut self, name: &String, declaration_index: DeclarationIndex, property_access: Vec<PropertyAccess>, optional_init: Option<Expr>, is_hoisting_allowed: bool) -> Option<()> {
        let trie = self.get_variable_trie();
        if let Some(cell) = trie.get_mut(name) {
            // Variable has been occured in this scope

            if is_hoisting_allowed {
                if cell.set(declaration_index, property_access, optional_init).is_err() {
                    // Variable has been declared, redeclaration is not allowed
                    return None
                }
            }

            Some(())
        } else {
            // Variable has not been occured in this scope
            let cell = VariablePointer::initialized(declaration_index, property_access, optional_init);
            trie.insert(name, cell.clone());

            Some(())
        }
    }

    fn visit_pattern(&mut self, pattern: Pattern, declaration_index: DeclarationIndex, is_hoisting_allowed: bool) -> Option<()> {
        let mut property_access = Vec::new();
        self.visit_pattern_internal(pattern, declaration_index, &mut property_access, is_hoisting_allowed)
    }

    fn visit_pattern_internal(&mut self, pattern: Pattern, declaration_index: DeclarationIndex, property_access: &mut Vec<PropertyAccess>, is_hoisting_allowed: bool) -> Option<()> {
        match pattern {
            Pattern::Rest(x) => unimplemented!("Rest pattern is not supported yet"),
            Pattern::Optional(name, init) => {
                self.declare(&name, declaration_index, property_access.clone(), Some(init), is_hoisting_allowed)
            }
            Pattern::ArrayPattern(elements) => {
                property_access.push(PropertyAccess::Element(0));
                let PropertyAccess::Element(access) = property_access.get_mut(property_access.len() - 1).unwrap();
                for element in elements {
                    self.visit_pattern_internal(element, declaration_index, property_access, is_hoisting_allowed)?;
                    *access += 1;
                }
                Some(())
            }
            Pattern::RecordPattern(props) => {
                property_access.push(PropertyAccess::Property(String::new()));
                let PropertyAccess::Property(access) = property_access.get_mut(property_access.len() - 1).unwrap();
                for prop in props {
                    match prop {
                        PropParam::Shorthand(name) => {
                            access.push_str(name.as_str());
                            self.declare(&name, declaration_index, property_access.clone(), None, is_hoisting_allowed)?;
                        }
                        PropParam::KeyValue(key, value) => {
                            access.push_str(key.as_str());
                            self.visit_pattern_internal(value, declaration_index, property_access, is_hoisting_allowed)?;
                        }
                        PropParam::Optional(name, init) => {
                            access.push_str(name.as_str());
                            self.declare(&name, declaration_index, property_access.clone(), Some(init), is_hoisting_allowed)?;
                        }
                        PropParam::Rest(name) => {
                            unimplemented!("Rest property is not supported yet")
                        }
                    }
                    access.clear();
                }
                Some(())
            }
            Pattern::Variable(name) => {
                self.declare(&name, declaration_index, property_access.clone(), None, is_hoisting_allowed)
            }
        }
    }

    pub fn declare_parameter(&mut self, pattern: Pattern) -> Option<DeclarationIndex> {
        let index = self.next_declaration_index();
        let decl = Declaration::Parameter { 
            pattern,
        };
        self.get_declarations().push(decl);

        self.visit_pattern(pattern, index, false)?;

        Some(index)
    }

    pub fn declare_let(&mut self, pattern: Pattern, value: Option<Expr>) -> Option<DeclarationIndex> {
        let index = self.next_declaration_index();
        let decl = Declaration::Let {
            pattern,
            value,
        };
        self.get_declarations().push(decl);

        self.visit_pattern(pattern, index, true)?;

        Some(index)
    }

    pub fn declare_const(&mut self, pattern: Pattern, value: Option<Expr>) -> Option<DeclarationIndex> {
        let index = self.next_declaration_index();
        let decl = Declaration::Const {
            pattern,
            value,
        };
        self.get_declarations().push(decl);

        self.visit_pattern(pattern, index, true)?;

        Some(index)
    }

    pub fn declare_function(&mut self, function: Function) -> Option<DeclarationIndex> {
        if function.name.is_none() {
            return None
        }

        let index = self.next_declaration_index();
        let decl = Declaration::Function {
            function: Box::new(function),
            index,
        };
        self.get_declarations().push(decl);

        self.declare(&function.name.unwrap(), index, Vec::new(), None, true)?;

        Some(index)
    }

    pub fn use_variable(&mut self, name: &String) -> Option<VariablePointer> {
        let trie = self.get_variable_trie();
        if let Some(ptr) = trie.get_mut(name) {
            Some(ptr.clone())
        } else {
            let ptr = VariablePointer::new();
            trie.insert(name, ptr);
            Some(ptr)
        }
    }

    pub fn take(mut self) -> (&'a mut Vec<Declaration>, Vec<(String, VariablePointer)>) {
        let mut unbound_uses = Vec::new();
        let trie = self.get_variable_trie();
        let ptrs = trie.iterate();
        for (name, ptr) in ptrs {
            if ptr.is_uninitialized() {
                unbound_uses.push((name.clone(), ptr));
            }
        }
        (self.declarations, unbound_uses)
    }

    pub fn assert_equivalence(&mut self, name: &String, mut var: VariablePointer) {
        let trie = self.get_variable_trie();
        if let Some(ptr) = trie.get_mut(name) {
            var.overwrite(ptr);
        } else {
            trie.insert(name, var);
        }
    }
}