use std::cell::{RefCell, OnceCell};

use jessie_ast::{Expr, VariableCell, Declaration, Pattern, PropertyAccess, PropParam, Variable, Function, DeclarationIndex, VariablePointer, PropertyAccessChain, OptionalPattern};
use utils::{Trie, OwnedSlice, OwnedString, SharedString};

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

    fn declare(&mut self, name: &SharedString, declaration_index: DeclarationIndex, property_access: PropertyAccessChain, is_hoisting_allowed: bool) -> Option<()> {
        let trie = self.get_variable_trie();
        if let Some(cell) = trie.get_mut(name) {
            // Variable has been occured in this scope

            if is_hoisting_allowed {
                if cell.set(declaration_index, property_access.clone()).is_err() {
                    // Variable has been declared, redeclaration is not allowed
                    return None
                }
            }

            Some(())
        } else {
            // Variable has not been occured in this scope
            let cell = VariablePointer::initialized(declaration_index, property_access.clone());
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
            Pattern::Optional(opt) => {
                unimplemented!("Optional pattern is not supported yet")
                /* 
                let OptionalPattern(_, jessie_ast::LValueOptional::Variable(var), default) = *opt;
                self.declare(&name, declaration_index, property_access, Some(init), is_hoisting_allowed)
                */
            }
            Pattern::ArrayPattern(elements) => {
                property_access.push(PropertyAccess::Element(0));
                let Some(PropertyAccess::Element(access)) = property_access.last_mut();
                for element in elements.0 {
                    self.visit_pattern_internal(element, declaration_index, property_access, is_hoisting_allowed)?;
                    *access += 1;
                }
                Some(())
            }
            Pattern::RecordPattern(props) => {
                property_access.push(PropertyAccess::Property(SharedString::empty()));
                let PropertyAccess::Property(access) = property_access.last_mut().unwrap();
                for prop in props.0 {
                    match prop {
                        PropParam::Shorthand(field, var) => {
                            let name = field.name();
                            *access = name; 
                            self.declare(&field.name(), declaration_index, PropertyAccessChain::from_vec(property_access.clone()), is_hoisting_allowed)?;
                        }
                        PropParam::KeyValue(field, value) => {
                            let name = field.name();
                            *access = name;
                            self.visit_pattern_internal(value, declaration_index, property_access, is_hoisting_allowed)?;
                        }
                        /* 
                        PropParam::Optional(name, init) => {
                            access.push_str(name.as_str());
                            self.declare(&name, declaration_index, property_access, Some(init), is_hoisting_allowed)?;
                        }
                        */
                        PropParam::Rest(name) => {
                            unimplemented!("Rest property is not supported yet")
                        }
                    }
                }
                Some(())
            }
            Pattern::Variable(var) => {
                // The variable declarations are already set by the caller, but it is 
                self.declare(&var.name, declaration_index, PropertyAccessChain::from_vec( property_access.clone()), is_hoisting_allowed)
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

    pub fn declare_optional_parameter(&mut self, name: SharedString, default: Expr) -> Option<DeclarationIndex> {
        let index = self.next_declaration_index();
        let decl = Declaration::OptionalParameter { name, default };
        self.get_declarations().push(decl);

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
        };
        self.get_declarations().push(decl);

        self.declare(&function.name.unwrap(), index, PropertyAccessChain::empty(), true)?;

        Some(index)
    }

    pub fn use_variable(&mut self, name: &SharedString) -> VariableCell {
        let trie = self.get_variable_trie();
        if let Some(ptr) = trie.get_mut(name) {
            ptr.clone().new_cell(name.clone())
        } else {
            let ptr = VariablePointer::new();
            trie.insert(name, ptr);
            ptr.new_cell(name.clone())
        }
    }

    pub fn take(mut self) -> (&'a mut Vec<Declaration>, OwnedSlice<(SharedString, VariablePointer)>) {
        let mut unbound_uses = Vec::new();
        let trie = self.get_variable_trie();
        let ptrs = trie.iterate();
        for (name, ptr) in ptrs {
            if ptr.is_uninitialized() {
                unbound_uses.push((name.clone(), ptr));
            }
        }
        (self.declarations, OwnedSlice::from_vec(unbound_uses))
    }

    pub fn assert_equivalence(&mut self, name: &SharedString, mut var: VariablePointer) {
        let trie = self.get_variable_trie();
        if let Some(ptr) = trie.get_mut(name) {
            var.overwrite(ptr);
        } else {
            trie.insert(name, var);
        }
    }
}