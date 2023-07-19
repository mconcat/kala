use std::cell::{RefCell, OnceCell};

use fxhash::FxHashMap;
use jessie_ast::{Expr, VariableCell, Pattern, PropertyAccess, PropParam, Variable, Function, DeclarationIndex, VariablePointer, PropertyAccessChain, OptionalPattern, LocalDeclaration, ParameterDeclaration, LValueOptional};
use utils::{SharedString, Map};
use crate::{map::VariablePointerMap, param};

#[derive(Debug)]
pub struct LexicalScope {
    pub declarations: Vec<LocalDeclaration>,
    pub variables: VariablePointerMap,
}

impl LexicalScope {
    pub fn new(declarations: Vec<LocalDeclaration>, variables: VariablePointerMap) -> Self {
        Self {
            declarations,
            variables,
        }
    }

    // Replaces both declarations and variable trie
    pub fn enter_function_scope(&mut self, variables: VariablePointerMap) -> Self {
        std::mem::replace(self, Self {
            declarations: Vec::new(),
            variables,
        })
    }

    pub fn exit_function_scope(&mut self, parent: Self) -> Self {
        std::mem::replace(self, parent)
    }

    // Preserves declarations, only resets variable trie
    pub fn replace_variable_map(&mut self, variables: VariablePointerMap) -> VariablePointerMap {
        std::mem::replace(&mut self.variables, variables)
    }

    fn next_declaration_index(&mut self) -> usize {
        self.declarations.len()
    }

    fn declare(&mut self, name: &SharedString, declaration_index: DeclarationIndex, property_access: PropertyAccessChain, is_hoisting_allowed: bool) -> Option<()> {
        if let Some(cell) = self.variables.get(name) {
            println!("declare exists {:?} {:?}", name, cell);
            // Variable has been occurred in this scope

            if is_hoisting_allowed {
                if cell.set(declaration_index, property_access.clone()).is_err() {
                    // Variable has been declared, re-declaration is not allowed
                    return None
                }
            } else {
                // XXX
                // for now, only the parameter declarations are not allowed to be hoisted,
                // however the use_variable call inside param() makes sort of "usage" before the declaration,
                // so we just proceed, super adhoc, need to be fixed later
                if cell.set(declaration_index, property_access.clone()).is_err() {
                    // Variable has been declared, re-declaration is not allowed
                    return Some(()) // XXXXXXX
                }

                // Is there even a case where a variable cannot be hoisted??? 

                // hoisting is not allowed, variable usage cannot come before the declaration
                // return None
            }

            Some(())
        } else {
            println!("declare not exists {:?}", name);
            // Variable has not been occurred in this scope
            let cell = VariablePointer::initialized(declaration_index, property_access.clone());
            self.variables.insert(name, cell.clone());

            Some(())
        }
    }

    fn visit_pattern(&mut self, pattern: &Pattern, declaration_index: DeclarationIndex, is_hoisting_allowed: bool) -> Option<()> {
        let mut property_access = Vec::new();
        self.visit_pattern_internal(pattern, declaration_index, &mut property_access, is_hoisting_allowed)
    }

    fn visit_pattern_internal(&mut self, pattern: &Pattern, declaration_index: DeclarationIndex, property_access: &mut Vec<PropertyAccess>, is_hoisting_allowed: bool) -> Option<()> {

        match pattern {
            Pattern::Rest(pattern) => {
                // directly visit the inner pattern
                self.visit_pattern_internal(pattern, declaration_index, property_access, is_hoisting_allowed)
            }
            Pattern::Optional(opt) => {
                // TODO: CPEAAPL approach has been changed from transmuting from lexer side analysis. 
                // We don't need to match the memory representations between pattern and expression, remove the first value later
                let OptionalPattern(_, LValueOptional::Variable(var), default) = opt;

                property_access.push(PropertyAccess::Property(var.name.clone()));

                self.declare(
                    &var.name, 
                    declaration_index.clone(), 
                    PropertyAccessChain::from_vec(property_access.clone()), 
                    is_hoisting_allowed,
                )?;

                property_access.pop();

                // recursively visit both the pattern and the default expression inside the Optional
                // adding any variables it contains to the current scope with the given declaration index.
                self.visit_pattern_internal(pattern, declaration_index.clone(), property_access, is_hoisting_allowed)?;
                // if it contains any variables, it will add them to the current scope with the given declaration index.
                self.visit_pattern_internal(&default, declaration_index, property_access, is_hoisting_allowed)
            }
            Pattern::ArrayPattern(elements) => {
                let mut index = 0;
                for element in &elements.0 {
                    property_access.push(PropertyAccess::Element(index));
                    self.visit_pattern_internal(element, declaration_index.clone(), property_access, is_hoisting_allowed)?;
                    property_access.pop();
                    index += 1;
                }
                Some(())
            }
            Pattern::RecordPattern(props) => {
                for prop in &props.0 {
                    match prop {
                        PropParam::Shorthand(field, var) => {
                            property_access.push(PropertyAccess::Property(field.clone()));
                            self.declare(&field.name(), declaration_index.clone(), PropertyAccessChain::from_vec(property_access.clone()), is_hoisting_allowed)?;
                            property_access.pop();
                        }
                        PropParam::KeyValue(field, value) => {
                            property_access.push(PropertyAccess::Property(field.clone()));
                            // This will add any variables it contains to the current scope with the given declaration index.
                            self.visit_pattern_internal(&value, declaration_index.clone(), property_access, is_hoisting_allowed)?;
                            property_access.pop();
                        }
                        PropParam::Rest(name) => {
                            property_access.push(PropertyAccess::Property(name.clone()));
                            self.visit_pattern_internal(
                                &name, 
                                declaration_index.clone(), 
                                property_access, 
                                is_hoisting_allowed,
                            )?;

                            property_access.pop();
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

    pub fn declare_parameter(&mut self, pattern: Pattern, result: &mut Vec<ParameterDeclaration>) -> Option<()> {
        let param_index = result.len();
        println!("declare_parameter {:?} {:?}", pattern, param_index);
        let decl = match pattern {
            Pattern::Optional(pat) => {
                let OptionalPattern(_, LValueOptional::Variable(var), default) = *pat;
                self.declare_optional_parameter(&var.name, default, param_index)?
            }
            Pattern::Rest(pat) => unimplemented!("Rest parameter is not supported yet"),
            Pattern::Variable(var) => self.declare_variable_parameter(&var.name, param_index)?,
            Pattern::ArrayPattern(_) => self.declare_pattern_parameter(&pattern, param_index)?,
            Pattern::RecordPattern(_) => self.declare_pattern_parameter(&pattern, param_index)?,
        };
        result.push(decl);
        Some(()) 
    }

    pub fn declare_parameters(&mut self, patterns: Vec<Pattern>, result: &mut Vec<ParameterDeclaration>) -> Option<()> {
        for pattern in patterns.into_iter() {
            self.declare_parameter(pattern, result)?;
        }
        Some(())
    }

    pub fn declare_variable_parameter(&mut self, name: &SharedString, index: usize) -> Option<ParameterDeclaration> {
        let decl = ParameterDeclaration::Variable { name: name.clone() };
        self.declare(name, DeclarationIndex::Parameter(index), PropertyAccessChain::empty(), false)?;

        Some(decl)
    }

    pub fn declare_pattern_parameter(&mut self, pattern: &Pattern, param_index: usize) -> Option<ParameterDeclaration> {
        let decl = ParameterDeclaration::Pattern { 
            pattern: pattern.clone(),
        };

        self.visit_pattern(&pattern, DeclarationIndex::Parameter(param_index), false)?;

        Some(decl)
    }

    pub fn declare_optional_parameter(&mut self, name: &SharedString, default: Expr, index: usize) -> Option<ParameterDeclaration> {
        let decl = ParameterDeclaration::Optional { name: name.clone(), default };
        self.declare(name, DeclarationIndex::Parameter(index), PropertyAccessChain::empty(), false)?;

        Some(decl)
    }

    pub fn declare_let(&mut self, pattern: &Pattern, value: Option<Expr>) -> Option<usize> {
        let index = self.next_declaration_index();

        self.visit_pattern(&pattern, DeclarationIndex::Local(index), true)?;

        let decl = LocalDeclaration::Let {
            pattern: pattern.clone(),
            value,
        };
        self.declarations.push(decl);

        Some(index)
    }

    pub fn declare_const(&mut self, pattern: Pattern, value: Option<Expr>) -> Option<usize> {
        let index = self.next_declaration_index();

        self.visit_pattern(&pattern, DeclarationIndex::Local(index), true)?;

        let decl = LocalDeclaration::Const {
            pattern,
            value,
        };
        self.declarations.push(decl);


        Some(index)
    }

    pub fn declare_function(&mut self, function: Function) -> Option<usize> {
        if function.name.is_none() {
            return None
        }

        let index = self.next_declaration_index();
        let function_name = function.name.clone();
        let decl = LocalDeclaration::Function {
            function: Box::new(function),
        };
        self.declarations.push(decl);

        self.declare(&function_name.unwrap(), DeclarationIndex::Local(index), PropertyAccessChain::empty(), true)?;

        Some(index)
    }

    pub fn use_variable(&mut self, name: &SharedString) -> VariableCell {
        if let Some(ptr) = self.variables.get(name) {
            println!("exists {:?} {:?}", name, ptr);
            ptr.clone().new_cell(name.clone())
        } else {
            println!("not exists {:?}", name);
            let ptr = VariablePointer::new();
            self.variables.insert(name, ptr.clone());
            ptr.new_cell(name.clone())
        }
    }

    pub fn assert_equivalence(&mut self, name: &SharedString, mut var: VariablePointer) {
        if let Some(ptr) = self.variables.get(name) {
            var.overwrite(ptr);
        } else {
            self.variables.insert(name, var);
        }
    }
}