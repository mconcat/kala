// Scoping logic
// Jessie does not have `var` declarations, so we only need to care 
// about function hoisting and let/const declarations

// The logic will be:
// 1. If you encounter a variable expression,
// 1-1. If it is not declared yet, push it to list of preaccessed variable
// 1-2. If it is declared in parent scopes, push it to list of preaccessed variable, with possible reference candidate to be the parent scope
// 1-3. If it is declared in current scope, set the reference to the current scope
// 2. If you encounter a function declaration,
// 2-1. If the preaccessed variable list has a variable with the same name, set the reference to the current scope, and delete it from the list
// 2-2. Set the reference to the current scope
// 3. If you encounter a let/const declaration,
// 3-1. If the preaccessed variable list has a variable with the same name, set the reference to TDZ variable, and delete it from the list
// 3-2. Set the reference to the current scope

// visual example:
/*
function main() {
    // let/const variable declared, add mapping from the name to the variable.
    let letDeclaredVariable = true;
    const constDeclaredVariable = true;
    // function declared, add mapping from the name to the function.
    function functionDeclaration() {

    }

    letDeclaredVariable; // accessing declared variable, link it to the variable
    constDeclaredVariable; // accessing declared variable, link it to the variable
    functionDeclaration(); // accessing declared function, link it to the function

    {
        letDeclaredVariable; // accessing (locally) undeclared variable, add it to the list of preaccess variable with possible candidate reference from the parent scope.
        constDeclaredVariable; // same here
        notDeclaredVariable; // accessing (globally) undeclared variable, add it to the list of preaccess variable.
        notyetDeclaredVariable; // accessing (globally) undeclared variable, add it to the list of preaccess variable.
        notyetDeclaredFunction;

        const constDeclaredVariable; // variable is declared, link the previous preaccess variable (inside the scope) to TDZ flag

        constDeclaredVariable; // accessing declared variable
    }
    // at this point, we know letDeclaredVariable and notDeclaredVariable is not defined in the scope.
    // enumerate the list of local preaccessed variables.
    // letDeclaredVariable has reference candidate, set to it.
    // notyetDeclaredFunction, notDeclaredVariable and notyetDeclaredVariable do not have any candidate, push it back to parent scope(it might be declared in parent scopes later).

    // we do not know yet whether it is declared as variable or function
    notyetDeclaredVariable; // accessing undeclared variable, add it to the list of preaccessed variables
    
    let notyetDeclaredVariable = true; // now the variable is declared, link all the previous references to TDZ flag

    notyetDeclaredVariable // accessing declared variable, link it to the variable 

    notyetDeclaredFunction(); // accessing undeclared variable, add it to the list of preaccessed variables

    function notyetDeclaredFunction() {} // link all the previous reference to the function(without TDZ flag, basically hoisting)

    notyetDeclaredFunction(); // accessing declared variable, link it to the variable 
}

// at this point, we know notDeclaredVariable is not defined in any scopes,
// so we can subst it to ReferenceError.




/////////////

// in the initial version, we will simply prohibit TDZ behaviors.
// so any variable accessed before the declaration is illegal
// unless they are declared as function(), in which case is a value hoisting

// visual example:
/*
function main() {
    // let/const variable declared, add mapping from the name to the variable.
    let letDeclaredVariable = true;
    const constDeclaredVariable = true;
    // function declared, add mapping from the name to the function.
    function functionDeclaration() {

    }

    letDeclaredVariable; // accessing declared variable, link it to the variable
    constDeclaredVariable; // accessing declared variable, link it to the variable
    functionDeclaration(); // accessing declared function, link it to the function

    {
        letDeclaredVariable; // accessing (locally) undeclared variable, but it is declared in parent scope, so refer to the parent.
        constDeclaredVariable; // same here
        notDeclaredVariable; // accessing (globally) undeclared variable, add it to the list of preaccess variable.
        notyetDeclaredVariable; // accessing (globally) undeclared variable, add it to the list of preaccess variable.
        notyetDeclaredFunction;

        const constDeclaredVariable; // variable is declared after the access, abort parsing. (normally TDZ)

        constDeclaredVariable; // accessing declared variable
    }
    // enumerate the list of local preaccessed variables.
    // notyetDeclaredFunction, notDeclaredVariable and notyetDeclaredVariable do not have any candidate, push it back to parent scope(it might be declared in parent scopes later).

    // we do not know yet whether it is declared as variable or function
    notyetDeclaredVariable; // accessing undeclared variable, add it to the list of preaccessed variables
    
    let notyetDeclaredVariable = true; // variable declared after the access, abort parsing. (normally TDZ)

    notyetDeclaredVariable // accessing declared variable, link it to the variable 

    notyetDeclaredFunction(); // accessing undeclared variable, add it to the list of preaccessed variables

    function notyetDeclaredFunction() {} // link all the previous reference to the function(without TDZ flag, basically hoisting)

    notyetDeclaredFunction(); // accessing declared variable, link it to the variable
}
// at this point, we know notDeclaredVariable is defined nowhere,
// and can abort parsing.
 */



*/

// All the above is detailed handling for TDZ and hoisting stuffs, I dont have enough time to implement it. So I will just implement the basic version of it.

use std::{rc::Rc, cell::RefCell, thread::scope};

use crate::trie::Trie;
pub struct LexicalScope {
    pub identifier: u32,
    pub parent: Option<Box<LexicalScope>>, // How can we remove box here, the parent scope is always in the stack, so we can just use reference. Using box for now because then I have to put lifetime parameter everywhere
    pub variables: Trie<Variable>,
    pub variable_identifier_counter: u32,
}

impl LexicalScope {
    fn has_variable(&self, name: &String) -> bool {
        self.variables.has(name)
    }

    fn get_variable(&self, name: &String) -> Option<Variable> {
        self.variables.get(name)
    }

    fn set_variable(&mut self, name: &String, variable: Variable) {
        self.variables.insert(name, variable);
    }

    fn get_variable_from_ancenstor(&self, name: &String) -> Option<Variable> {
        let mut scope = self;
        while let Some(parent) = &scope.parent {
            if let Some(variable) = parent.get_variable(name) {
                return Some(variable);
            }
            scope = parent;
        }
        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VariableStatus {
    Declared, // Declared in the current scope, fixed.
    DeclaredInParent, // Declared in the parent scope, possibly TDZ or hoisting
    Undeclared, // Undeclared in any of the scope chain, possibly TDZ or hoisting
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variable(Rc<RefCell<VariableInternal>>);

#[derive(Clone, Debug, PartialEq)]
pub enum VariableInternal {
    Declared(u32, u32),
    DeclaredInParent(u32, Variable),
    Undeclared(u32, u32),
}

impl Variable {
    pub fn new_local(declared: bool, declared_scope: u32, local_variable_index: u32) -> Self {
        if declared {
            Self(Rc::new(RefCell::new(VariableInternal::Declared(declared_scope, local_variable_index))))
        } else {
            Self(Rc::new(RefCell::new(VariableInternal::Undeclared(declared_scope, local_variable_index))))
        }
    }

    pub fn declared_scope(&self) -> u32 {
        match &*self.0.borrow() {
            VariableInternal::Declared(scope, _) => *scope,
            VariableInternal::DeclaredInParent(scope, _) => *scope,
            VariableInternal::Undeclared(scope, _) => *scope,
        }
    }

    pub fn new_parent(declared_scope: u32, var: Variable) -> Self {
        Self(Rc::new(RefCell::new(VariableInternal::DeclaredInParent(declared_scope, var))))
    }

}

pub struct ScopingState {
    pub current_scope: LexicalScope,
    pub scope_identifier_counter: u32,
}

impl ScopingState {
    pub fn new() -> Self {
        Self {
            current_scope: LexicalScope {
                identifier: 0,
                parent: None,
                variables: Trie::empty(),
                variable_identifier_counter: 0,
            },
            scope_identifier_counter: 0,
        }
    }

    fn new_scope_identifier(&mut self) -> u32 {
        self.scope_identifier_counter += 1;
        self.scope_identifier_counter
    }
    
    fn current_scope_identifier(&self) -> u32 {
        self.current_scope.identifier
    }

    fn new_variable_identifier(&mut self) -> u32 {
        self.current_scope.variable_identifier_counter += 1;
        self.current_scope.variable_identifier_counter
    }

    fn local_variable(&mut self) -> VariableInternal {
        VariableInternal::Declared(self.current_scope_identifier(), self.new_variable_identifier())
    }

    fn declare_local_variable(&mut self) -> Variable {
        Variable::new_local(true, self.current_scope_identifier(), self.new_variable_identifier())
    }

    fn use_parent_variable(&mut self, parent_variable: Variable) -> Variable {
        // if parent variable is used, the only case it can be overwritten is 
        // when a function is declared in the current scope with the same name.
        // when a variable is declared in the current scope with the same name, it is illegal.
        // if no declaration is found until exit we can safely use the parent variable.
        Variable::new_parent(self.current_scope_identifier(), parent_variable)
    }

    fn use_undeclared_variable(&mut self) -> Variable {
        // if undeclared variable is used, the only case it can be overwritten is 
        // when a function is declared in the current scope with the same name.
        // when a variable is declared in the current scope with the same name, it is illegal.
        // if no declaration is found until exit, it is illegal.
        Variable::new_local(false, self.current_scope_identifier(), self.new_variable_identifier())
    }



    // Declare a variable in the current scope.
    // local let/const/function declarations
    // hoisting semantics of function does not effect on linking the variable to the scope,
    // the interpreter just need to call the function instead of return TDZ.
    pub fn def_variable(&mut self, name: &String) -> Result<Variable, String> {
        if let Some(var) = self.current_scope.get_variable(name) {
            match *var.0.borrow() {
                VariableInternal::Declared(_, _) => {
                    // Variable is declared in the current scope, throw error
                    return Err(format!("Variable {} is already declared in the current scope", name));
                },
                VariableInternal::DeclaredInParent(_, _) => {
                    // Variable has been declared in the parent scope, and it has been already used in the current scope.
                    // override the link to the parent and use the local variable instead
                    // TDZ
                    *var.0.borrow_mut() = self.local_variable();
                    return Ok(var);
                },
                VariableInternal::Undeclared(_, _) => {
                    // Variable has been used in the current scope, and not declared in the parent scope.
                    // TDZ
                    *var.0.borrow_mut() = self.local_variable();
                    return Ok(var);
                },
            }
        }

        // Variable is not yet used in the current scope, we can safely use shadowing variable regardless of the existence of the parent variable.

        let var = self.declare_local_variable();
        self.current_scope.set_variable(name, var);
        Ok(var)
    }

    pub fn use_variable(&mut self, name: &String) -> Result<Variable, String> {
        if let Some(var) = self.current_scope.get_variable(name) {
            // the variable is either declared or used in the current scope before, use it
            return Ok(var)
        }

        // The variable is not (yet) locally declared from this point.
        if let Some(parent) = self.current_scope.get_variable_from_ancenstor(name) {
            // the variable is declared in parents, refer it.
            return Ok(self.use_parent_variable(parent))
        }

        // the variable is not undeclared anywhere, use it as undeclared.
        return Ok(self.use_undeclared_variable());
    }

    pub fn enter_block(self) -> Self {
        let scope_identifier = self.scope_identifier_counter + 1;

        // 1. Create a new scope, set as child, and set as current scope
        let child = LexicalScope {
            identifier: scope_identifier,
            parent: Some(Box::new(self.current_scope)),
            variables: Trie::empty(),
            variable_identifier_counter: 0,
        };

        Self {
            current_scope: child,
            scope_identifier_counter: scope_identifier,
        }
    }

    pub fn exit_block(self) -> Result<Self, String> {
        let new_current_scope = self.current_scope.parent.ok_or("Cannot exit block from the global scope")?;

        // before we discard the current scope, we need to check if there is any parent-declared or undeclared variables left in the current scope.
        // if there is, we need to link them to the parent scope.
        // they could be either TDZ variables, or hoisted functions, or parent declared, or undeclared variables(which is error).
        // parent-declared variables should be passed up until it reaches the scope where the parent has been declared. in the process, we check if we meet a declaration for the name, in which case is a TDZ.
        // undeclared variables should be passed up until it reaches the global scope. in the process, we check if we meet a declaration for the name, in which case is a TDZ.

        // 1. iterate through the current scope's variables, and filter out the variables that are not declared in the current scope.
        let local_vars = self.current_scope.variables.iterate();
        let undeclared_local_vars = local_vars.into_iter().filter_map(|(name, var)| {
            match *var.0.borrow() {
                VariableInternal::Declared(_, _) => None,
                VariableInternal::DeclaredInParent(_, parent) => {
                    // if the scope where the parent is declared is the current scope, it is not TDZ.
                    // if the scope where the parent is declared is not the current scope, it could be TDZ, keep passing up.
                    if parent.declared_scope() == new_current_scope.identifier {
                        // we have reached where the parent is declared
                        // no need to keep propagating up.
                        None
                    } else {
                        // we have not reached where the parent is declared
                        // keep propagating up.
                        Some((name, var))
                    }
                },
                VariableInternal::Undeclared(_, _) => {
                    // Variable has been used in the current scope, and not declared in the parent scope.
                    // TDZ, propagate up until it meets a declaration
                    Some((name, var))
                },
            }
        });

        // 2. push the undeclared variables to the parent scope. 

        if let Some(parent) = self.current_scope.parent {
            Ok(Self {
                current_scope: *parent,
                scope_identifier_counter: self.scope_identifier_counter,
            })
        } else {
            Err("Cannot exit the global scope".to_string())
        }
    }
}

/* 
#[derive(Debug, PartialEq, Clone)]
pub struct Variable(Rc<RefCell<VariableInternal>>);

#[derive(Debug, PartialEq, Clone)]
pub enum VariableInternal {
    Preaccess(String, Option<Variable>),
    Variable(String, ScopeID), // once Variable, forever Variable
}

impl Variable {
    pub fn is_preaccess(&self) -> bool {
        match *self.0.borrow() {
            VariableInternal::Preaccess(_, _) => true,
            _ => false,
        }
    }

    pub fn is_undeclared(&self) -> bool {
        match *self.0.borrow() {
            VariableInternal::Preaccess(_, None) => true,
            _ => false,
        }
    }

    pub fn is_possibly_declared(&self) -> bool {
        match *self.0.borrow() {
            VariableInternal::Preaccess(_, Some(_)) => true,
            _ => false,
        }
    }

    pub fn is_declared(&self) -> bool {
        match *self.0.borrow() {
            VariableInternal::Variable(_, _) => true,
            _ => false,
        }
    }
}
*/

/* 

#[derive(Debug, PartialEq, Clone)]
pub struct ScopeID(u32); // for now

// Scope is a struct used during parsing
// It manages hoisting, scoping, and identifier binding.
#[derive(Debug, PartialEq, Clone)]
pub struct Scope {
    pub id: ScopeID,

    // list of preaccess variables i.e. used before declaration
    // the list contains all the preaccess variable in the current scope 
    // including the children scopes.
    // when the scope is popped, the list is enumerated to check if any of the 
    // candidates are defined at the parent scope, and if so, apply candidates.
    // For the remainings, push them back to the parent scope. 
    pub preaccess: Vec<Variable>,

    // list of variables declared in the exact current scope.
    // this list will be used to "flush" the awating bindings
    // when the scope is popped.
    // pub declarations: Vec<Variable>,

    // binding is the mapping from the variable identifiers to the known reference.
    // if the variable is declared in the current scope, the reference is set to the variable.
    // if the variable is declared in the parent scope, the reference is set to the variable,
    // but it will be still in preaccess list(to check TDZ behavior or hoisted function).
    // if the variable is not declared yet, the reference will be set to null, 
    // and will be stored in preaccess.
    // when any of the variable/function declaration is encountered,
    // and there is no existing bindings yet, it will be stored as bindings.
    // if there are preaccess variable exist for the declared name,
    // pop the preaccess from the list, and set the binding, only if the declaration is function.
    // at the end of each block, the binding trie will be iterated, to do the following:
    // 1. if the variable is a preaccess, 
    // 1-1. if the candidate is from the direct parent, and if so, apply the candidate to be bind. 
    // 1-2. if the direct parent has the same name defined, 
    pub binding: Trie<Variable>,

    pub parent: Option<Box<Scope>>,
}

impl Scope {
    pub fn empty() -> Scope {
        Scope {
            id: ScopeID(0), // TODO

            preaccess: Vec::new(),
            binding: Trie::empty(),
            parent: None,
        }
    }

    pub fn get_local_binding(&mut self, name: &String) -> Option<Variable> {
        self.binding.get(name)
    } 

    pub fn get_parent_binding(&mut self, name: &String) -> Option<Variable> {
        if let Some(parent) = &mut self.parent {
            parent.get_local_binding(name).or_else(|| parent.get_parent_binding(name))
        } else {
            None
        }
    }

    // parent have declared it, but it might be TDZ
    // add to bindings but also to preaccess
    pub fn make_possibly_preaccess_variable(&mut self, name: &String, candidate: Variable) -> Variable {
        let var = Variable(Rc::new(RefCell::new(VariableInternal::Preaccess(name.clone(), Some(candidate)))));
        self.preaccess.push(var);
        self.binding.insert(name, var.clone());
        var
    }

    // no existing declaration yet
    // add to bindings but also to preaccess
    pub fn make_preaccess_variable(&mut self, name: &String) -> Variable {
        let var = Variable(Rc::new(RefCell::new(VariableInternal::Preaccess(name.clone(), None))));
        self.preaccess.push(var);
        self.binding.insert(&name, var.clone());
        var
    }

    // variable is not yet defined in the current scope, define it.
    pub fn define_variable(&mut self, name: &String) -> Variable {
        let var = Variable(Rc::new(RefCell::new(VariableInternal::Variable(name.clone(), self.id))));
        self.binding.insert(&name, var.clone());
        var
    }
}
*/