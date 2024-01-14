use std::{rc::Rc, thread::Scope, cell::RefCell};

use jessie_ast::{Variable, Declaration, Function, Module, Block, Pattern, PropParam, OptionalPattern, LValueOptional, VariableIndex};
use utils::{Map, MapPool};

use crate::scope_expression;

//type VariableMapPool = MapPool<Variable>;
type VariableMap = Map<Variable>;

//type VariablePointerMapPool<'a> = MapPool<&'a mut Variable>;
type VariablePointersMap<'a> = Map<Vec<&'a mut Variable>>;

#[derive(Debug)]
pub struct ScopeState {
    builtins: VariableMap,
    module_scope: ModuleScope,
}

impl ScopeState {
    pub fn new() -> Self {
        Self {
            builtins: VariableMap::default(),
            module_scope: ModuleScope {
                global_scope: BlockScope {
                    declared_variables: VariableMap::default(),
                },
                function_scopes: Vec::new(),
            }
        }
    }

    fn current_function(&mut self) -> Option<&mut FunctionScope> {
        self.module_scope.function_scopes.last_mut()
    }

    pub fn use_variable(&mut self, var: &mut Variable) -> Result<(), &'static str> {
        // Recursively search for the variable, in reverse order, for each function scope
        // If the function scope has the variable, the variable is replaced with the declared variable.
        // If not, we replace the variable with a capture variable, which points to the (not yet initialized) parent variable.
        // We repeat the step with the parent variable, until we reach the global scope.

        println!("use_variable: {:?}", var);

        let mut declared_var = Variable::new(var.name.clone());

        let mut found_where = None;
        for (i, func) in self.module_scope.function_scopes.iter_mut().enumerate().rev() {
            if func.use_variable(&mut declared_var).is_ok() {
                found_where = Some(i);
                break
            }
        };

        // declared_var is now pointing to the variable where the variable is declared

        println!("found_where for {:?}: {:?}", var, found_where);
        match found_where {
            None => {
                if let Some(global_var) = self.module_scope.global_scope.declared_variables.get(&var.name) {
                    *var = global_var.clone();
                    Ok(())
                } else if let Some(static_var) = self.builtins.get(&var.name) {
                    *var = static_var.clone();
                    Ok(())
                } else {
                    Err("Variable not declared")
                }
            }
            Some(func_index) => {
                let mut parent = declared_var.clone();
                let current_function = self.module_scope.function_scopes.len();
                for func in self.module_scope.function_scopes.as_mut_slice()[func_index+1..current_function].iter_mut() {
                    parent = func.declare_capture(parent)?;
                }
                *var = parent;

                Ok(())
            }
        }
    }

    pub fn declare_function(&mut self, func: Rc<RefCell<Function>>) -> Result<(), &'static str> {
        self.current_function().unwrap().declare_function(func)
    }
    
    pub fn declare_let_variable(&mut self, var: &mut Variable) -> Result<(), &'static str> {
        self.current_function().unwrap().declare_variable(var)
    }

    pub fn declare_const_variable(&mut self, var: &mut Variable) -> Result<(), &'static str> {
        self.declare_let_variable(var) // TODO
    }

    pub fn declare_parameter(&mut self, var: &mut Variable) -> Result<(), &'static str> {
        self.current_function().unwrap().declare_parameter(var)
    }

    pub fn declare_pattern(&mut self, pattern: &mut Pattern, mut f: impl FnMut(&mut Self, &mut Variable) -> Result<(), &'static str>) -> Result<(), &'static str> {
        match pattern {
            Pattern::Variable(var) => f(self, var.as_mut()),
            Pattern::ArrayPattern(arr) => arr.0.iter_mut().try_for_each(|pat| self.declare_pattern(pat, f)),
            Pattern::RecordPattern(rec) => rec.0.iter_mut().try_for_each(|prop| {
                match prop {
                    PropParam::KeyValue(_, value) => self.declare_pattern(value, f),
                    PropParam::Shorthand(key, var) => f(self, var),
                    PropParam::Rest(var) => f(self, var)
                }
            }),
            Pattern::Optional(pat) => {
                match pat.as_mut() {
                    OptionalPattern(_, LValueOptional::Variable(left), right) => {
                        f(self, left.as_mut())?;
                        scope_expression(self, right)
                    }
                }
            }
            Pattern::Rest(pattern) => self.declare_pattern(pattern, f),
        }
    }

    pub fn enter_function(&mut self, func: &mut Function) -> Result<(), &'static str>{
        self.module_scope.function_scopes.push(FunctionScope{
            locals: Vec::new(),
            captures: Vec::new(),
            functions: Vec::new(),

            block_scopes: vec![BlockScope{declared_variables: VariableMap::default()}],
        });

        for param in func.parameters.iter_mut() {
            self.declare_pattern(param, Self::declare_parameter)?;
        }

        Ok(())
    }

    pub fn exit_function(&mut self) -> jessie_ast::FunctionScope {
        let scope = self.module_scope.function_scopes.pop().unwrap();

        jessie_ast::FunctionScope{
            locals: scope.locals.into(),
            captures: scope.captures.into(),
            functions: scope.functions.into(),
        }
    }

    pub fn enter_block(&mut self, block: &mut Block) -> Result<(), &'static str> {
        let mut declared_variables = VariableMap::with_capacity_and_hasher(block.statements.len(), Default::default());
        self.current_function().unwrap().block_scopes.push(BlockScope{
            declared_variables
        });

        for decl in block.declarations.iter_mut() {
            match decl {
                Declaration::Const(decls) => {
                    for decl in decls.iter_mut() {
                        self.declare_pattern(&mut decl.pattern)?;
                    }
                }
                Declaration::Let(decls) => {
                    for decl in decls.iter_mut() {
                        self.declare_pattern(&mut decl.pattern)?;
                    }
                }
                Declaration::Function(func) => {
                    self.declare_function(func.clone())?;
                }
            }
        }

        Ok(())
    }

    pub fn exit_block(&mut self) {
        self.current_function().unwrap().block_scopes.pop();
    }
}

#[derive(Debug)]
pub struct ModuleScope {
    pub global_scope: BlockScope,

    pub function_scopes: Vec<FunctionScope>,

}

#[derive(Debug)]
pub struct FunctionScope {
    pub locals: Vec<Variable>,
    pub captures: Vec<Variable>,
    pub functions: Vec<(Variable, Rc<RefCell<Function>>)>,
    
    pub block_scopes: Vec<BlockScope>,
}

impl FunctionScope {
    fn current_block(&mut self) -> &mut BlockScope {
        self.block_scopes.last_mut().unwrap()
    }

    fn use_variable(&mut self, var: &mut Variable) -> Result<(), &'static str> {
        for block in self.block_scopes.iter_mut().rev() {
            if let Some(declared_var) = block.declared_variables.get_mut(&var.name) {
                *var = declared_var.clone();
                return Ok(());
            }
        }

        Err("Variable not declared")
    }

    fn declare_let_variable(&mut self, var: &mut Variable) -> Result<(), &'static str> {
        self.declare_variable(var, VariableIndex::Local(false, self.locals.len().try_into().unwrap()))
    }


    fn declare_const_variable(&mut self, var: &mut Variable) -> Result<(), &'static str> {
        self.declare_variable(var, VariableIndex::Local(true, self.locals.len().try_into().unwrap()))
    }

    fn declare_parameter(&mut self, var: &mut Variable) -> Result<(), &'static str> {
        self.declare_variable(var, VariableIndex::Parameter(self.parameters.len().try_into))
    }

    fn declare_variable(&mut self, var: &mut Variable, index: VariableIndex) -> Result<(), &'static str> {
        if self.current_block().declared_variables.contains_key(&var.name) {
            return Err("Variable already declared");
        }
        var.pointer.set(index);
        self.locals.push(var.clone());
        self.current_block().declared_variables.insert(var.name.clone(), var.clone());
        Ok(())
    }


    fn declare_function(&mut self, func: Rc<RefCell<Function>>) -> Result<(), &'static str> {
        let name = func.borrow().get_name().unwrap();
        if self.current_block().declared_variables.contains_key(&name) {
            return Err("Function already declared");
        }
        let var = Variable::declared(name.clone(), VariableIndex::Local(self.locals.len().try_into().unwrap()));
        self.locals.push(var.clone());
        self.current_block().declared_variables.insert(name, var.clone());
        self.functions.push((var, func.clone()));
        Ok(())
    }
    
    fn declare_capture(&mut self, parent: Variable) -> Result<Variable, &'static str> {
        let capture_index = self.captures.len();
        self.captures.push(parent.clone());
        let var = Variable::declared(parent.name.clone(), VariableIndex::Captured(capture_index.try_into().unwrap()));
        Ok(var)
    }
}

#[derive(Debug)]
pub struct BlockScope {
    pub declared_variables: VariableMap,
}