use std::{io::empty, rc::{Rc, self}};

use jessie_ast::{Variable, Declaration, Pattern, Expr, variable, VariableIndex, PropParam, LValueOptional, Function, Block, VariableDeclaration};
use utils::{SharedSlice, SharedString, Map, MapPool, VectorMap, FxMap};

use crate::map::{VariableMap, VariableMapPool, self};

#[derive(Debug)]
pub struct ModuleScope {
    pub function_scopes: Vec<FunctionScope>,

    map_pool: VariableMapPool,
}

impl ModuleScope {
    pub fn new() -> Self {
        let mut map_pool = VariableMapPool::new();

        ModuleScope {
            // implicity initial top level function scope
            function_scopes: vec![FunctionScope{
                functions: vec![],
                declarations: vec![],
                declared_variables: vec![],
                block_scopes: vec![BlockScope{
                    variables_map: map_pool.get(),
                    block_id: 0,
                }],
                block_id: 0,
            }],
            map_pool,
        }
    }

    pub fn declare_const(&mut self, decl: Vec<(Pattern, Option<Expr>)>) -> Option<Declaration> {
        self.get_function_scope().declare_const(decl)
    }

    pub fn declare_let(&mut self, decl: Vec<(Pattern, Option<Expr>)>) -> Option<Declaration> {
        self.get_function_scope().declare_let(decl)
    }

    pub fn declare_function(&mut self, function: Function) -> Option<Declaration> {
        self.get_function_scope().declare_function(function)
    }

    pub fn use_variable(&mut self, name: SharedString) -> Variable {
        self.get_function_scope().use_variable(name)
    }

    pub fn use_variable_as_captured(&mut self, name: SharedString) -> Variable {
        let var = self.use_variable(name);
        
    }

    fn get_function_scope(&mut self) -> &mut FunctionScope {
        self.function_scopes.last_mut().unwrap()
    }

    pub fn enter_function(&mut self) {
        let mut function_scope = FunctionScope {
            functions: vec![],
            declarations: vec![],
            declared_variables: vec![],
            block_scopes: vec![],
            block_id: 0,
        };

        self.function_scopes.push(function_scope);

        self.enter_block();
    }

    pub fn initialize_parameters(&mut self, parameters: &Vec<Pattern>) {
        let mut param_index = 0;

        for param in parameters {
            match param {
                Pattern::Variable (var) => {
                    match self.get_function_scope().get_variable(&var.name).unwrap().index.get() {
                        VariableIndex::Unknown => {
                            var.set_parameter_index(param_index);
                        }
                        VariableIndex::Parameter(_) => unimplemented!("Duplicate parameter"),
                        VariableIndex::Local(_) => unreachable!("Local variable cannot be parameter"),
                        VariableIndex::Capture(_) => unreachable!("Captured variable cannot be parameter"), 
                        VariableIndex::Static(_) => unreachable!("Static variable cannot be parameter"),
                    }
                    var.set_parameter_index(param_index);

                    // function_scope.block_scope().variables_map.insert(var.name, Variable::new(var.name, VariableIndex::Parameter(param_index), function_scope.current_block_id())).unwrap()
                    param_index += 1;
                }
                _ => unimplemented!("Pattern parameter"),
            };

        }
    }

    pub fn exit_function(&mut self) -> (Vec<(Variable, Rc<Function>)>, Vec<Variable>, Vec<Variable>) {
        let BlockScope{mut variables_map, ..} = self.get_function_scope().block_scopes.pop().unwrap();
        let FunctionScope{mut declarations, mut declared_variables, functions, ..} = self.function_scopes.pop().unwrap();

        let mut captures = vec![];

        println!("######exit_function");
        for (_, var) in self.map_pool.drain(variables_map) {
            println!("######exit_function iter {:?}", var);
            match var.index.get() {
                VariableIndex::Unknown => {
                    var.set_capture_index(captures.len() as u32);
                    
                    let captured_var = self.get_function_scope().use_variable(var.name.clone());

                    captures.push(captured_var); 
                }
                _ => (),
            }
        }

        (functions, declared_variables, captures)
    }
    
    pub fn enter_block(&mut self) {
        let variables_map = self.map_pool.get();

        let block_scope = BlockScope {
            variables_map,
            block_id: self.get_function_scope().block_id,
        };

        self.get_function_scope().block_scopes.push(block_scope);

        self.get_function_scope().block_id += 1;
    }

    pub fn exit_block(&mut self) {
        let mut block_scope = self.get_function_scope().block_scopes.pop().unwrap();

        for (name, mut var) in self.map_pool.drain(block_scope.variables_map) {
            if var.index.get() == VariableIndex::Unknown {
                if let Some(existing_var) = self.get_function_scope().get_variable(&name) {
                    // IT SHOULD BE A DOUBLE POINTER
                    // If any error occurs when having more than one nested block scopes, check here
                    var.index = existing_var.index.clone();
                } else {
                    self.get_function_scope().block_scope().variables_map.insert(name, var);
                }
            }
        }
    }

    pub fn exit_module<T: Clone>(&mut self, builtins: &mut FxMap<T>) -> Vec<T> {
        assert!(self.function_scopes.len() == 1);

        let BlockScope{mut variables_map, ..} = self.get_function_scope().block_scopes.pop().unwrap();
        let FunctionScope{mut declarations, mut declared_variables, functions, ..} = self.function_scopes.pop().unwrap();

        // settle undeclared(capture) variables
        let mut used_builtins: Vec<T> = Vec::new();
        let mut used_builtins_map: FxMap<Variable> = FxMap::new();

        for (_, mut used_var) in self.map_pool.drain(variables_map) {
            if used_var.index.get() != VariableIndex::Unknown {
                // pass if already bound
                continue
            }
            if let Some(builtin_var) = used_builtins_map.get(used_var.name.clone()) {
                used_var.index.set(builtin_var.index.get());
                continue
            }
            let index = used_builtins.len();
            used_var.set_static_index(index as u32);
            used_builtins.push(builtins.get(used_var.name.clone()).unwrap().clone());
            used_builtins_map.insert(used_var.name.clone(), used_var);
        }

        used_builtins
    }
}

#[derive(Debug)]
pub struct FunctionScope {
    pub declarations: Vec<Declaration>,
    pub functions: Vec<(Variable, Rc<Function>)>,
    pub declared_variables: Vec<Variable>, // as declared order, corresponds to VariableIndex::Local
    pub block_scopes: Vec<BlockScope>,
    pub block_id: u32, // current block scope id, introduced to distinguish variables with same name across different scopes
}

#[derive(Debug)]
pub struct BlockScope {
    pub block_id: u32,
    pub variables_map: VariableMap, // any variables occured in this scope, regardless of declaration
}

impl FunctionScope {
    fn block_scope(&mut self) -> &mut BlockScope {
        self.block_scopes.last_mut().unwrap()
    }

    fn current_block_id(&self) -> u32 {
        self.block_scopes.last().unwrap().block_id
    }

    fn get_variable(&mut self, name: &SharedString) -> Option<&mut Variable> {
        self.block_scope().variables_map.get(name.clone())
    }

    fn hoist_declare(&mut self, name: SharedString) -> Variable {
        let var = Variable::unknown(name.clone(), self.current_block_id());
        self.block_scope().variables_map.insert(name, var.clone());
        println!("\n\n\nhoist_declare: {:?}", var.clone());
        var
    }

    pub fn use_variable(&mut self, name: SharedString) -> Variable {
        println!("use_variable: {:?}", self.block_scopes);
        if let Some(var) = self.get_variable(&name) {
            println!("\n\n\n\nuse_variable: {:?}", var.clone());
            var.clone()
        } else {
            self.hoist_declare(name)
        }
    }

    fn declare_variable(&mut self, var: &mut Variable) -> Option<()> {
        if let Some(existing_var) = self.get_variable(&var.name).cloned() {
            match existing_var.index.get() {
                VariableIndex::Unknown => {
                    // Variable has been used before declaration
                    existing_var.set_local_index(self.declared_variables.len().try_into().unwrap());
                    self.declared_variables.push(existing_var.clone());
                    var.index.set(existing_var.index.get());
                    println!("\n\n\n\ndeclare_variable_hoist {:?}, {:?}", existing_var, var);
                    return Some(())
                }
                _ => {
                    // Variable has been declared before
                    // duplicate declaration
                    return None;
                }
            }
        }


        // declared for the first time
        var.set_local_index(self.declared_variables.len().try_into().unwrap());
        self.declared_variables.push(var.clone());
        self.block_scope().variables_map.insert(var.name.clone(), var.clone());
        println!("\n\n\n\ndeclare_variable {:?}", var);
        Some(())
    }

    fn declare_item(&mut self, pattern: &mut Pattern) -> Option<()> {
        match pattern {
            Pattern::Variable(ref mut var) => {
                self.declare_variable(var)?;
                Some(())
            }
            Pattern::ArrayPattern(items) => {
                for item in &mut items.0 {
                    self.declare_item(item)?;
                }
                Some(())
            }
            Pattern::RecordPattern(items) => {
                for item in &mut items.0 {
                    match item {
                        PropParam::KeyValue(key, ref mut value) => self.declare_item(value),
                        PropParam::Shorthand(key, ref mut var) => self.declare_variable(var),
                        PropParam::Rest(ref mut var) => self.declare_variable(var)
                    }?

                }
                Some(())
            }
            Pattern::Optional(pattern) => {
                match &mut pattern.1 {
                    LValueOptional::Variable(ref mut var) => {
                        self.declare_variable(var)
                    }
                }
            }
            Pattern::Rest(pattern) => {
                self.declare_item(&mut *pattern)
            }
        }
    }
    // similar with declare_variable but does not check existing declaration and does not push to the declared_variables
    pub fn declare_parameter(&mut self, var: &mut Variable) {
    }
        

    pub fn declare_function(&mut self, function: Function) -> Option<Declaration> {
        let mut var = Variable::unknown(function.name.get_name().unwrap().clone(), self.current_block_id());
        self.declare_variable(&mut var)?;

        let rc_function = Rc::new(function);

        self.functions.push((var, rc_function.clone()));

        Some(Declaration::Function(rc_function))
    }

    pub fn declare_const(&mut self, pattern: Vec<(Pattern, Option<Expr>)>) -> Option<Declaration> {
        let mut decls = Vec::with_capacity(pattern.len());
        for (ref mut item, init) in pattern {
            self.declare_item(item)?;
            decls.push(VariableDeclaration{pattern: item.clone(), value: init.clone()})
        }
        Some(Declaration::Const(Rc::new(decls)))
    }

    pub fn declare_let(&mut self, pattern: Vec<(Pattern, Option<Expr>)>) -> Option<Declaration> {
        let mut decls = Vec::with_capacity(pattern.len());
        for (ref mut item, init) in pattern {
            self.declare_item(item)?;
            decls.push(VariableDeclaration{pattern: item.clone(), value: init.clone()})
        }
        Some(Declaration::Let(Rc::new(decls)))
    }
}