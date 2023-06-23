use std::cell::{RefCell, OnceCell};

use fxhash::FxHashMap;
use jessie_ast::{Expr, VariableCell, Declaration, Pattern, PropertyAccess, PropParam, Variable, Function, DeclarationIndex, VariablePointer, PropertyAccessChain, OptionalPattern};
use utils::{OwnedSlice, OwnedString, SharedString};
use std::collections::HashMap;

type Map<T> = fxhash::FxHashMap<SharedString, T>;

#[derive(Debug, PartialEq)]
pub struct LexicalScope {
    pub declarations: Vec<Declaration>,
    pub variable_trie: Option<Box<Map<VariablePointer>>>,
}

impl LexicalScope {
    pub fn new(declarations: Vec<Declaration>) -> Self {
        Self {
            declarations,
            variable_trie: None,
        }
    }

    // Replaces itself with an empty lexical scope(parent declarations and empty variable trie)
    // Returns lexical scope with empty declarations and parent variable trie
    pub fn replace_with_child(&mut self) -> Self {
        let parent_variable_trie = std::mem::replace(&mut self.variable_trie, None);

        Self {
            declarations: Vec::new(),
            variable_trie: parent_variable_trie,
        }
    }

    pub fn recover_parent(&mut self, parent: Self) {
        std::mem::replace(&mut self.variable_trie, parent.variable_trie);
    }

    fn get_declarations(&mut self) -> &mut Vec<Declaration> {
        &mut self.declarations
    }

    fn get_variable_trie(&mut self) -> &mut Map<VariablePointer> {
        if self.variable_trie.is_none() {
            self.variable_trie = Some(Box::new(HashMap::with_hasher(Default::default())));
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
            trie.insert(name.clone(), cell.clone());

            Some(())
        }
    }

    fn visit_pattern(&mut self, pattern: &Pattern, declaration_index: DeclarationIndex, is_hoisting_allowed: bool) -> Option<()> {
        let mut property_access = Vec::new();
        self.visit_pattern_internal(pattern, declaration_index, &mut property_access, is_hoisting_allowed)
    }

    fn visit_pattern_internal(&mut self, pattern: &Pattern, declaration_index: DeclarationIndex, property_access: &mut Vec<PropertyAccess>, is_hoisting_allowed: bool) -> Option<()> {

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
                let mut index = 0;
                for element in elements.0.as_ref() {
                    property_access.push(PropertyAccess::Element(index));
                    self.visit_pattern_internal(element, declaration_index.clone(), property_access, is_hoisting_allowed)?;
                    property_access.pop();
                    index += 1;
                }
                Some(())
            }
            Pattern::RecordPattern(props) => {
                for prop in props.0.as_ref() {
                    match prop {
                        PropParam::Shorthand(field, var) => {
                            property_access.push(PropertyAccess::Property(field.clone()));
                            self.declare(&field.name(), declaration_index.clone(), PropertyAccessChain::from_vec(property_access.clone()), is_hoisting_allowed)?;
                            property_access.pop();
                        }
                        PropParam::KeyValue(field, value) => {
                            property_access.push(PropertyAccess::Property(field.clone()));
                            self.visit_pattern_internal(&value, declaration_index.clone(), property_access, is_hoisting_allowed)?;
                            property_access.pop();
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

        self.visit_pattern(&pattern, index.clone(), false)?;

        let decl = Declaration::Parameter { 
            pattern,
        };
        self.get_declarations().push(decl);



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

        self.visit_pattern(&pattern, index.clone(), true)?;

        let decl = Declaration::Let {
            pattern,
            value,
        };
        self.get_declarations().push(decl);

        Some(index)
    }

    pub fn declare_const(&mut self, pattern: Pattern, value: Option<Expr>) -> Option<DeclarationIndex> {
        let index = self.next_declaration_index();

        self.visit_pattern(&pattern, index.clone(), true)?;

        let decl = Declaration::Const {
            pattern,
            value,
        };
        self.get_declarations().push(decl);


        Some(index)
    }

    pub fn declare_function(&mut self, function: Function) -> Option<DeclarationIndex> {
        if function.name.is_none() {
            return None
        }

        let index = self.next_declaration_index();
        let function_name = function.name.clone();
        let decl = Declaration::Function {
            function: Box::new(function),
        };
        self.get_declarations().push(decl);

        self.declare(&function_name.unwrap(), index.clone(), PropertyAccessChain::empty(), true)?;

        Some(index)
    }

    pub fn use_variable(&mut self, name: &SharedString) -> VariableCell {
        let trie = self.get_variable_trie();
        if let Some(ptr) = trie.get_mut(name) {
            ptr.clone().new_cell(name.clone())
        } else {
            let ptr = VariablePointer::new();
            trie.insert(name.clone(), ptr.clone());
            ptr.new_cell(name.clone())
        }
    }

    pub fn take(mut self) -> (Vec<Declaration>, OwnedSlice<(SharedString, VariablePointer)>) {
        let mut unbound_uses = Vec::new();
        let trie = self.get_variable_trie();
        let ptrs = trie.drain();
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
            trie.insert(name.clone(), var);
        }
    }
}