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
/* 
    // Replaces itself with an empty lexical scope(parent declarations and empty variable trie)
    // Returns lexical scope with empty declarations and parent variable trie
    pub fn replace_with_child(&mut self, variables: Map<VariablePointer>) -> Self {
        let parent_variables = std::mem::replace(&mut self.variables, variables);

        Self {
            declarations: Vec::new(),
            variables: parent_variables,
        }
    }

    pub fn recover_parent(&mut self, parent: Self) -> Map<VariablePointer> {
        std::mem::replace(&mut self.variables, parent.variables)
    }
*/
    fn next_declaration_index(&mut self) -> DeclarationIndex {
        DeclarationIndex::Local(self.declarations.len())
    }

    fn declare(&mut self, name: &SharedString, declaration_index: DeclarationIndex, property_access: PropertyAccessChain, is_hoisting_allowed: bool) -> Option<()> {
        if let Some(cell) = self.variables.get(name) {
            println!("declare exists {:?} {:?}", name, cell);
            // Variable has been occured in this scope

            if is_hoisting_allowed {
                if cell.set(declaration_index, property_access.clone()).is_err() {
                    // Variable has been declared, redeclaration is not allowed
                    return None
                }
            } else {
                // XXX
                // for now, only the parameter declarations are not allowed to be hoisted,
                // however the use_variable call inside param() makes sort of "usage" before the declaration,
                // so we just proceed, super adhoc, need to be fixed later
                if cell.set(declaration_index, property_access.clone()).is_err() {
                    // Variable has been declared, redeclaration is not allowed
                    return None
                }

                // Is there even a case where a variable cannot be hoisted??? 

                // hoisting is not allowed, variable usage cannot come before the declaration
                // return None
            }

            Some(())
        } else {
            println!("declare not exists {:?}", name);
            // Variable has not been occured in this scope
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

    pub fn declare_parameter(&mut self, pattern: Pattern, result: &mut Vec<ParameterDeclaration>) -> Option<()> {
        let param_index = DeclarationIndex::Parameter(result.len());
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

    pub fn declare_variable_parameter(&mut self, name: &SharedString, index: DeclarationIndex) -> Option<ParameterDeclaration> {
        let decl = ParameterDeclaration::Variable { name: name.clone() };
        self.declare(name, index, PropertyAccessChain::empty(), false)?;

        Some(decl)
    }

    pub fn declare_pattern_parameter(&mut self, pattern: &Pattern, param_index: DeclarationIndex) -> Option<ParameterDeclaration> {
        let decl = ParameterDeclaration::Pattern { 
            pattern: pattern.clone(),
        };

        self.visit_pattern(&pattern, param_index.clone(), false)?;

        Some(decl)
    }

    pub fn declare_optional_parameter(&mut self, name: &SharedString, default: Expr, index: DeclarationIndex) -> Option<ParameterDeclaration> {
        let decl = ParameterDeclaration::Optional { name: name.clone(), default };
        self.declare(name, index, PropertyAccessChain::empty(), false)?;

        Some(decl)
    }

    pub fn declare_let(&mut self, pattern: &Pattern, value: Option<Expr>) -> Option<DeclarationIndex> {
        let index = self.next_declaration_index();

        self.visit_pattern(&pattern, index.clone(), true)?;

        let decl = LocalDeclaration::Let {
            pattern: pattern.clone(),
            value,
        };
        self.declarations.push(decl);

        Some(index)
    }

    pub fn declare_const(&mut self, pattern: Pattern, value: Option<Expr>) -> Option<DeclarationIndex> {
        let index = self.next_declaration_index();

        self.visit_pattern(&pattern, index.clone(), true)?;

        let decl = LocalDeclaration::Const {
            pattern,
            value,
        };
        self.declarations.push(decl);


        Some(index)
    }

    pub fn declare_function(&mut self, function: Function) -> Option<DeclarationIndex> {
        if function.name.is_none() {
            return None
        }

        let index = self.next_declaration_index();
        let function_name = function.name.clone();
        let decl = LocalDeclaration::Function {
            function: Box::new(function),
        };
        self.declarations.push(decl);

        self.declare(&function_name.unwrap(), index.clone(), PropertyAccessChain::empty(), true)?;

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