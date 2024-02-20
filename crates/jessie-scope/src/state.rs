use std::{cell::RefCell, rc::Rc, thread::{current, Scope}};

use jessie_ast::{Block, Declaration, Function, LValueOptional, LocalVariable, Module, OptionalPattern, Pattern, PropParam, Variable, VariableIndex};
use utils::{Map, MapPool};

use crate::scope_expression;

//type VariableMapPool = MapPool<Variable>;
type VariableMap = Map<Variable>;

//type VariablePointerMapPool<'a> = MapPool<&'a mut Variable>;
type VariablePointersMap<'a> = Map<Vec<&'a mut Variable>>;

#[derive(Debug, Clone)]
pub struct BuiltinMap<T> {
    builtins: Map<T>,
    used: Vec<T>,
    variables: Map<Variable>,
}

impl<T> BuiltinMap<T> {
    pub fn new() -> Self {
        Self {
            builtins: Map::default(),
            used: Vec::new(),
            variables: Map::default(),
        }
    }
}

impl<T: Clone> BuiltinMap<T> {
    fn get(&mut self, name: &str) -> Option<Variable> {
        if self.variables.contains_key(name) {
            return self.variables.get(name).cloned();
        }
        let res = self.builtins.get(name);
        if res.is_none() {
            return None;
        }
        let slot = res.unwrap();
        let index = self.used.len();
        self.used.push(slot.clone());
        let var = Variable::declared(name.into(), VariableIndex::Static(index.try_into().unwrap()));
        self.variables.insert(name.into(), var.clone());
        Some(var)
    }
}

#[derive(Debug)]
pub struct ScopeState<T> {
    builtins: BuiltinMap<T>,
    module_scope: ModuleScope,
}

impl<T: Clone> ScopeState<T> {
    pub fn used_builtins(&self) -> Vec<T> {
        self.builtins.used.clone()
    }

    pub fn empty() -> Self {
        Self {
            builtins: BuiltinMap::new(),
            module_scope: ModuleScope {
                global_scope: BlockScope {
                    declared_variables: VariableMap::default(),
                },
                function_scopes: Vec::new(),
            }
        }
    }

    pub fn new(builtins: Map<T>) -> Self {
        Self {
            builtins: BuiltinMap {
                builtins,
                used: Vec::new(),
                variables: Map::default(),
            },
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
                if func_index == self.module_scope.function_scopes.len() - 1 {
                    // the variable is declared in the current function, no need to capture
                    *var = declared_var;
                    return Ok(());
                }

                let current_function = self.module_scope.function_scopes.len();

                // add to the escapings of the function where the variable is declared
                let function_where_declared = self.module_scope.function_scopes.as_mut_slice().get_mut(func_index).unwrap();
                match declared_var.index() {
                    VariableIndex::Parameter(index) => {
                        let var_decl = &mut function_where_declared.parameters.as_mut_slice()[index as usize];
                        var_decl.is_escaping = true;
                    },
                    VariableIndex::Local(_, index) => {
                        let var_decl = &mut function_where_declared.locals.as_mut_slice()[index as usize];
                        var_decl.is_escaping = true;
                    },
                    // Static variables are always heap allocated
                    VariableIndex::Static(_) => {},
                    // Captured variables does not need to be declared as escaping
                    VariableIndex::Captured(_) => {},
                }

                // recursively declare the variable as a capture for each function between the current function and the function where the variable has been declared
                let mut parent = declared_var.clone();
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
        let current_function = self.current_function().unwrap();
        
        current_function.declare_variable(var, VariableIndex::Local(false, current_function.locals.len() as u32))
    }

    pub fn declare_const_variable(&mut self, var: &mut Variable) -> Result<(), &'static str> {
        let current_function = self.current_function().unwrap();
        
        current_function.declare_variable(var, VariableIndex::Local(true, current_function.locals.len() as u32))
    }

    pub fn declare_parameter(&mut self, var: &mut Variable) -> Result<(), &'static str> {
        let current_function = self.current_function().unwrap();

        current_function.declare_variable(var, VariableIndex::Parameter(current_function.parameters.len() as u32))?;

        current_function.parameters.push(LocalVariable::new(var.clone()));

        Ok(())
    }

    pub fn declare_pattern(&mut self, pattern: &mut Pattern, f: &mut impl FnMut(&mut Self, &mut Variable) -> Result<(), &'static str>) -> Result<(), &'static str> {
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

    // using function as a wrapper for the script
    // hacky, TODO fix
    pub fn enter_script(&mut self) -> Result<(), &'static str> {
        if self.module_scope.function_scopes.len() > 0 {
            return Err("Script already entered");
        }

        self.module_scope.function_scopes.push(FunctionScope{
            parameters: Vec::new(), 
            locals: Vec::new(),
            captures: Vec::new(),
            functions: Vec::new(),

            block_scopes: vec![BlockScope{declared_variables: VariableMap::default()}],
        });

        Ok(())
    }

    pub fn exit_script(&mut self) -> Result<Box<[LocalVariable]>, &'static str> {
        let scope = self.module_scope.function_scopes.pop().unwrap(); 

        if scope.parameters.len() > 0 {
            panic!("script should not have parameters");
        }
        if scope.captures.len() > 0 {
            return Err("script should not have captures, some variables are not scoped and not in the builtins");
        }

        Ok(scope.locals.into()) // should we also return the functions? for hoisting? TODO
    }

    pub fn enter_function(&mut self, func: &mut Function) -> Result<(), &'static str>{
        self.module_scope.function_scopes.push(FunctionScope{
            parameters: Vec::with_capacity(func.parameters.len()),
            locals: Vec::new(),
            captures: Vec::new(),
            functions: Vec::new(),

            block_scopes: vec![BlockScope{declared_variables: VariableMap::default()}],
        });

        for param in func.parameters.iter_mut() {
            self.declare_pattern(param, &mut Self::declare_parameter)?;
        }

        Ok(())
    }

    pub fn exit_function(&mut self) -> jessie_ast::FunctionScope {
        let scope = self.module_scope.function_scopes.pop().unwrap();

        jessie_ast::FunctionScope{
            parameters: scope.parameters.into(),
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
                        self.declare_pattern(&mut decl.pattern, &mut Self::declare_const_variable)?;
                    }
                }
                Declaration::Let(decls) => {
                    for decl in decls.iter_mut() {
                        self.declare_pattern(&mut decl.pattern, &mut Self::declare_let_variable)?;
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
    pub parameters: Vec<LocalVariable>,
    pub locals: Vec<LocalVariable>,
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

        for parameter in self.parameters.iter() {
            if parameter.var.name == var.name {
                *var = parameter.var.clone();
                return Ok(());
            }
        }

        Err("Variable not declared")
    }


    fn declare_variable(&mut self, var: &mut Variable, index: VariableIndex) -> Result<(), &'static str> {
        if self.current_block().declared_variables.contains_key(&var.name) {
            return Err("Variable already declared");
        }
        var.pointer.set(index);
        self.locals.push(LocalVariable::new(var.clone()));
        self.current_block().declared_variables.insert(var.name.clone(), var.clone());
        Ok(())
    }


    fn declare_function(&mut self, func: Rc<RefCell<Function>>) -> Result<(), &'static str> {
        let name = func.borrow().get_name().unwrap();
        if self.current_block().declared_variables.contains_key(&name) {
            return Err("Function already declared");
        }
        let var = Variable::declared(name.clone(), VariableIndex::Local(true, self.locals.len().try_into().unwrap()));
        self.locals.push(LocalVariable::new(var.clone()));
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