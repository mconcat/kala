// Scoping logic for singlepass parser
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

use std::{rc::Rc, cell::RefCell};

use crate::trie::Trie;

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