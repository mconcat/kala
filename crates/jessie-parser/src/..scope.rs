use std::{cell::{RefCell, OnceCell}, rc::Rc};

use fxhash::FxHashMap;
use jessie_ast::{Expr, Pattern, PropertyAccess, PropParam, VariableIndex, Function, DeclarationIndex, VariablePointer, OptionalPattern, ParameterDeclaration, LValueOptional};
use utils::{SharedString, Map, FxMap};
use crate::{map::{VariablePointerMap, VariableMap}};

pub struct DeclarationVisitor<'a> {
    scope: &'a mut LexicalScope,
    is_hoisting_allowed: bool,
}

impl<'a> PatternVisitor for DeclarationVisitor<'a> {
    fn visit(&mut self, index: DeclarationIndex, name: SharedString, property_access: Vec<PropertyAccess>) -> Option<()> {
        self.scope.declare(name.clone(), index, &property_access, self.is_hoisting_allowed)
    }
}



#[derive(Debug)]
pub struct LexicalScope {
    pub variables: VariableMap,
}

impl LexicalScope {
    pub fn new(parameters: Vec<ParameterDeclaration>, variables: VariablePointerMap) -> Self {
        Self {
            declarations,
            variables,
        }
    }

    // Replaces both declarations and variable trie
    pub fn enter_function_scope(&mut self, variables: VariablePointerMap) -> Self {
        std::mem::replace(self, Self {
            declarations: FunctionDeclarations::empty(),
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

    fn declare(&mut self, name: SharedString, declaration_index: DeclarationIndex, property_access: &Vec<PropertyAccess>, is_hoisting_allowed: bool) -> Option<()> {
        println!("declare environment variables: {:?}", self.variables);
        if let Some(cell) = self.variables.get(name.clone()) {
            if cell.is_uninitialized() {
                println!("declare uninitialized {:?} {:?}", name, cell);
                // variable is in its first occurance
                return cell.set(declaration_index, property_access.clone()).ok()
            }

            println!("declare exists {:?} {:?}", name, cell);
            // Variable has been occured in this scope
            // hoisting

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
                    return Some(()) // XXXXXXX
                }

                // Is there even a case where a variable cannot be hoisted??? 

                // hoisting is not allowed, variable usage cannot come before the declaration
                // return None
            }

            Some(())
        } else {
            unreachable!("variable should have been occured in the left side of the declaration, logic error")
            /* 
            println!("declare not exists {:?}", name);
            // Variable has not been occured in this scope
            let cell = VariablePointer::initialized(declaration_index, &property_access);
            self.variables.insert(name, cell.clone());

            Some(())
            */
        }
    }
/* 
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
                
            }
        }
    }
*/
    pub fn declare_parameter(&mut self, pattern: Pattern, result: &mut Vec<ParameterDeclaration>) -> Option<()> {
        let param_index = result.len() as u32;
        println!("declare_parameter {:?} {:?}", pattern, param_index);
        let decl = match pattern {
            Pattern::Optional(pat) => {
                let OptionalPattern(_, LValueOptional::Variable(var), default) = *pat;
                self.declare_optional_parameter(var.name, default, param_index)?
            }
            Pattern::Rest(pat) => unimplemented!("Rest parameter is not supported yet"),
            Pattern::Variable(var) => self.declare_variable_parameter(var.name, param_index)?,
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

    pub fn declare_variable_parameter(&mut self, name: SharedString, index: u32) -> Option<ParameterDeclaration> {
        let decl = ParameterDeclaration::Variable { name: name.clone() };
        self.declare(name, DeclarationIndex::Parameter(index as u32), &vec![], false)?;

        Some(decl)
    }

    pub fn declare_pattern_parameter(&mut self, pattern: &Pattern, param_index: u32) -> Option<ParameterDeclaration> {
        let decl = ParameterDeclaration::Pattern { 
            pattern: pattern.clone(),
        };

        pattern.visit(DeclarationIndex::Parameter(param_index), &mut DeclarationVisitor {
            scope: self,
            is_hoisting_allowed: false,
        })?;

        Some(decl)
    }

    pub fn declare_optional_parameter(&mut self, name: SharedString, default: Expr, index: u32) -> Option<ParameterDeclaration> {
        let decl = ParameterDeclaration::Optional { name: name.clone(), default };
        self.declare(name, DeclarationIndex::Parameter(index), &vec![], false)?;

        Some(decl)
    }

    pub fn declare_let(&mut self, pattern: &Pattern, value: Option<Expr>) -> Option<(u32, Rc<LocalDeclaration>)> {
        let index = self.declarations.locals.len() as u32;

        println!("declare_let {:?} {:?}", pattern, index);
        pattern.visit(DeclarationIndex::Local(index as u32), &mut DeclarationVisitor {
            scope: self,
            is_hoisting_allowed: true,
        })?;

        let decl = Rc::new(LocalDeclaration::Let {
            pattern: pattern.clone(),
            value,
        });
        self.declarations.locals.push(decl.clone());

        Some((index, decl))
    }

    pub fn declare_const(&mut self, pattern: Pattern, value: Expr) -> Option<(u32, Rc<LocalDeclaration>)> {
        let index = self.declarations.locals.len() as u32;

        pattern.visit(DeclarationIndex::Local(index as u32), &mut DeclarationVisitor {
            scope: self,
            is_hoisting_allowed: true,
        })?;

        let decl = Rc::new(LocalDeclaration::Const {
            pattern,
            value,
        });
        self.declarations.locals.push(decl.clone());


        Some((index, decl))
    }

    pub fn declare_function(&mut self, function: Function) -> Option<(u32, Rc<LocalDeclaration>)> {
        if !function.name.is_named() {
            return None
        }

        let index = self.declarations.locals.len() as u32;
        let function_name = function.name.clone();
        let decl = Rc::new(LocalDeclaration::Function {
            function: Box::new(function),
        });
        self.declarations.locals.push(decl.clone());

        self.declare(function_name.get_name().unwrap().clone(), DeclarationIndex::Local(index as u32), &vec![], true)?;

        Some((index, decl))
    }

    pub fn use_variable(&mut self, name: &SharedString) -> VariableCell {
        if let Some(ptr) = self.variables.get(name.clone()) {
            println!("exists {:?} {:?}", name, ptr);
            ptr.clone().new_cell(name.clone())
        } else {
            println!("not exists {:?}", name);
            let ptr = VariablePointer::new();
            self.variables.insert(name.clone(), ptr.clone());
            ptr.new_cell(name.clone())
        }
    }

    pub fn assert_equivalence(&mut self, name: SharedString, mut var: VariablePointer) {
        if let Some(ptr) = self.variables.get(name.clone()) {
            var.overwrite(ptr);
        } else {
            self.variables.insert(name.clone(), var);
        }
    }
}