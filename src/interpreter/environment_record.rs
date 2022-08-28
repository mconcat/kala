use crate::runtime::JSValue;
use core::panic;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Variable<V: JSValue> {
    Local(V, bool),
    Captured(Rc<RefCell<V>>, bool),
}

impl<V: JSValue> Variable<V> {
    #[inline]
    pub fn new(value: V, mutable: bool) -> Self {
        Self::Local(
            value,
            mutable,
        )
    }
    
    #[inline]
    pub fn capture(&mut self) -> &Self {
        // capture applies only on the closure capture and should be distinguished with heap allocation
        // (all objects are heap allocated by default, regardless of their escaping)
        // as capturing applies to the variable, not the value, we don't need to recursively
        // capture the inner references.
        match self {
            Self::Local(value, mutable) => {
                *self = Self::Captured(
                    Rc::new(RefCell::new(value.clone())),
                    *mutable,
                )
            },
            _ => {},
        };
        self
    }

    pub fn get(&self) -> &V {
        match self {
            Self::Local(value, _) => value,
            Self::Captured(value, _) => &(*(value.borrow_mut())).clone(),
        }
    }

    pub fn modify(&mut self, f: impl Fn(&mut V)) {
        match self {
            Self::Local(value, _) => f(value),
            Self::Captured(value, _) => f(&mut *(*value).borrow_mut()),
        }
    }

    pub fn set(&mut self, value: V) {
        match self {
            Self::Local(existing_value, _) => *existing_value = value,
            Self::Captured(existing_value, _) => *existing_value.borrow_mut() = value,
        }
    }

    pub fn is_mutable(&self) -> bool {
        match self {
            &Self::Local(_, mutable) => mutable,
            &Self::Captured(_, mutable) => mutable,
        }
    }
/*
    pub fn depth(&self) -> i16 {
        match self {
            &Self::Stack(_, _, depth) => depth,
            &Self::Heap(_, _, depth) => depth,
        }
    }*/

}
use hashbrown::HashMap;

// EnvironmentRecord is a local scope variable binding.
#[derive(Debug)]
pub struct EnvironmentRecord<V: JSValue> {
    // parent scope
    // TODO: ownership belongs to the root environment only. 
    // remove rc and replace with a refcell to the parent.
    parent: Option<Rc<RefCell<EnvironmentRecord<V>>>>,

    // Stack emulates stack memory by creating a stack of local variables.
    // The variables can have local lifetime(Variable::Local)
    // or can be captured by an escaping closure(Variable::Captured)
    //
    // NOTE: we will replace the variable name to a static i32 index AOT,
    // so to make the transition easier, we use a separate hashmap to map 
    // the variable name to the local index.
    //
    // The type of stack is ManuallyDrop<Vec> because the underlying vector should 
    // be allocated by the creator and should live along the execution of the program
    // which means the EnvironmentRecord should not destruct the stack.
    stack: RefCell<Vec<Variable<V>>>,
     
    // Starting index of the local scope
    // TODO: remove start by statically learn variable indicies
    start: usize,

    // IndexMap is a local variable mapping 
    // TODO: remove indexmap by statically learn variable indicies
    indexmap: HashMap<String, isize>,
}

impl<V: JSValue> Clone for EnvironmentRecord<V> {
    fn clone(&self) -> Self {
        Self {
            parent: self.parent.clone(),
            stack: self.stack, // shared
            start: self.start,
            indexmap: self.indexmap.clone(),
        }
    }
}

impl<V: JSValue> Drop for EnvironmentRecord<V> {
    fn drop(&mut self) {
        drop(self.parent);
        // EnvironmentRecord does not drops the entire stack,
        // but only the segment that it owns as a local scope.
        if self.start == 0 {
            drop(self.stack)
        } else {
            let stack = self.stack.borrow_mut();
            for i in self.start..stack.len() {
               drop(stack[i])
            }
            unsafe { stack.set_len(self.start) }
        }
        
        drop(self.indexmap);
        drop(self.start)
    }
}

impl<V: JSValue> EnvironmentRecord<V> {
    pub fn new(global_bindings: Vec<(String, Variable<V>)>, global_stack: Vec<Variable<V>>) -> Self {
        let mut root = EnvironmentRecord {
            parent: None,
            stack: RefCell::new(global_stack),
            indexmap: HashMap::new(),
            start: global_stack.len(), 
        };

        for (name, var) in global_bindings {
            root.push_variable(&name, var)
        }
        
        root
    }

    pub fn enter(&mut self, captures: Vec<&String>, parameters: Vec<(&String, V)>) -> Self {
        let child = EnvironmentRecord { 
            parent: Some(Rc::new(RefCell::new(*self))),
            stack: self.stack, 
            indexmap: HashMap::new(), 
            start: self.stack.borrow().len(),
        };
        for capture in captures {
            let absolute_index = self.get_variable_index_recursive(capture).expect("captured undeclared variable");
            child.indexmap.insert(capture.to_string(), self.to_relative_index(absolute_index));
        }
        for (paramname, param) in parameters {
            child.push_variable(paramname, Variable::new(param, true));
        }
        child
    }

    fn get_variable_index_local(&self, name: &String) -> Option<usize> {
        if let Some(index) = self.indexmap.get(name) {
            let stack = self.stack.borrow();
            let absolute_index = self.to_absolute_index(*index);
            Some(absolute_index) 
        } else { None }
    }
    
    fn get_variable_index_recursive(&self, name: &String) -> Option<usize> {
        self.get_variable_index_local(name).or_else(|| 
            self.parent.and_then(|parent| {
                parent.borrow_mut().get_variable_index_recursive(name)
            })
        )
    }

    fn to_relative_index(&self, absolute_index: usize) -> isize {
        absolute_index as isize - self.start as isize
    }

    // ASSERT: |relative_index| <= self.start
    fn to_absolute_index(&self, relative_index: isize) -> usize {
        self.start + relative_index as usize
    }

    fn cache_variable_index(&mut self, name: &String, relative_index: isize) {
        self.indexmap.insert(name.to_string(), relative_index);
    }

    fn variable_by_index(&mut self, absolute_index: usize) -> &mut Variable<V> {
        &mut self.stack.borrow_mut()[absolute_index]
    }

    fn find_variable(&mut self, name: &String) -> Option<&mut Variable<V>> {
        if let Some(absolute_index) = self.get_variable_index_local(name) {
            Some(self.variable_by_index(absolute_index))
        } else if let Some(absolute_index) = self.parent.and_then(|parent| parent.borrow().get_variable_index_recursive(name)) {
            let relative_index = self.to_relative_index(absolute_index);
            self.cache_variable_index(name, relative_index);
            Some(self.variable_by_index(absolute_index))
        } else {
            None
        }
    }

    fn push_variable(&mut self, name: &String, v: Variable<V>) {
        let index = self.stack.borrow().len()-self.start;
        if index >= self.stack.borrow().capacity() {
            panic!("stack overflow")
        }
        self.stack.borrow_mut().push(v);
        self.indexmap.insert(name.clone(), index as isize);
    }

    pub fn initialize_mutable_binding(&mut self, name: &String, value: &V) {
        // this shadows the existing variable and block any further access by the variable name
        // but if any closure captured the existing variable it should be still accessible
        self.push_variable(name, Variable::new(value.clone(), true))
    }

    pub fn initialize_immutable_binding(&mut self, name: &String, value: &V) -> Result<(), String> {
        // check for any duplicate local variable
        if self.indexmap.contains_key(name) {
            return Err(format!("redeclaration of formal parameter {}", name))
        }
        self.push_variable(name, Variable::new(value.clone(), false));
        Ok(())
    }

    pub fn get_binding_value(&self, name: &String) -> Option<&V> {
        self.find_variable(name).map(|var| var.get())
    }

    pub fn set_mutable_binding(&mut self, name: &String, v: &V) -> Result<(), String> {
        let var = self.find_variable(name);
        if var.is_none() {
            return Err(format!("ReferenceError: assignment to undeclared variable \"{}\"", name))
        }
        let var = var.unwrap();

        if !var.is_mutable() {
            return Err("TypeError: Assignment to constant variable".to_string())
        }

        var.set(v.clone());

        Ok(())
    }

    #[inline]
    pub fn modify_mutable_binding(&mut self, name: &String, f: impl Fn(&mut V)) -> Result<(), String> {
        let var = self.find_variable(name);
        if var.is_none() {
            return Err(format!("ReferenceError: assignment to undeclared variable \"{}\"", name))
        }
        let var = var.unwrap();

        if !var.is_mutable() {
            return Err("TypeError: Assignment to constant variable".to_string())
        }

        var.modify(f);

        Ok(())
    }
/*
    // variable_mut should be de
    #[inline]
    pub fn get_mutable_binding(&mut self, name: &String) -> Result<&mut Variable<V>, String> {
        let index = self.indexmap.get(name);
        if index.is_none() {
            return Err()
        }

        let var = self.access_variable_by_index(index);
        if !var.is_mutable() {
            return Err()
        }

        match var {
            Variable::Local(var, _) => {
                var
            },
            Variable::Captured(var, _) => {
                &mut *var.borrow_mut()
            }
        }
    }
*/
/*
    pub fn declare(&mut self, name: &String, kind: ast::DeclarationKind, value: Option<JSValue>) -> Result<(), String> {
        if kind == ast::DeclarationKind::Const && value.is_none() {
            panic!("const variable must be initialized");
        }
        
        let binding = self.binding.get(name).clone();

        // if there is no variable existing already, create a new one, and add it to the recovery list as to be discarded
        if binding.is_none() {
            self.add_recover_variable(name.clone(), None);
            let var = EnvironmentRecord<JSValue>dVariable::new(value.unwrap_or(JSValue::Undefined), kind, self.depth());
            self.binding.insert(name.clone(), var); 
            return Ok(())
        }
        
        let existing = binding.unwrap();
        let existing_kind = existing.kind();

        // if there is a variable existing already, check if it is a parent variable, 
        // if so, shadow it by adding it to the recovery list as to be recovered, and create a new one
        if self.is_parent_variable(existing) {
            let recover_value = Some(existing.clone());
            self.add_recover_variable(name.clone(), recover_value);
        }

        // at this point the variable is already declared in the current scope, so we just update it after checking if it is a let
        else if existing_kind != ast::DeclarationKind::Let {
            // cannot set to non-let variable
            return 
        }

        let var = EnvironmentRecord<JSValue>dVariable::new(value.unwrap_or(JSValue::Undefined), kind, self.depth());
        self.binding.insert(name.clone(), var);
        Ok(())
    }


    pub fn exit(&mut self) {
        if let Some(recovery) = self.recovery.pop() {
            for (key, entry) in recovery.iter() {
                match entry {
                    Some(existing) => {
                        self.binding.insert(key.clone(), existing.clone());
                    }
                    None => {
                        self.binding.remove(key);
                    }
                }
            }
        } else {
            panic!("exit scope without entering");
        }
    }

    */
}

#[cfg(test)]
mod scope_tests {
    use crate::environment_record::EnvironmentRecord;
    use crate::interpreter::runtime::JSValue;
    
    #[test]
    fn scope_test_simple() {
        let global_stack = Vec::with_capacity(100);

        let scope = &mut EnvironmentRecord::new(vec![], global_stack);

        let declare_let = |scope: &mut EnvironmentRecord<JSValue>, name: &str, value: Option<JSValue>| {
            scope.initialize_mutable_binding(&name.to_string(), &value.unwrap_or(JSValue::undefined()))
        };

        let declare_const = |scope: &mut EnvironmentRecord<JSValue>, name: &str, value: Option<JSValue>| {
            scope.initialize_immutable_binding(&name.to_string(), &value.unwrap_or(JSValue::undefined()))
        };

        let set_var = |scope: &mut EnvironmentRecord<JSValue>, name: &str, value: JSValue| {
            scope.set_mutable_binding(&name.to_string(), &value)
        };

        let assert_error = |scope: &mut EnvironmentRecord<JSValue>, name: &str| {
            assert!(scope.set_mutable_binding(&name.to_string(), &JSValue::undefined()).is_err());
        };

        let assert_var = |scope: &mut EnvironmentRecord<JSValue>, name: &str, value: JSValue| {
            assert_eq!(scope.get_binding_value(&name.to_string()).unwrap(), &value);
        };

        /*
        {
            let a = 1;
            const b = 2;
            a = 3;
            // b = 4; // Error!
            let c;
            c = 4;
            {
                const a = 11;
                let b = 12;
                let x = 13;
                c = 14;
            }
            a = 5;
            // b = 6; // Error!
        }
        */

        println!("1");
        declare_let(scope, "a", Some(JSValue::number(1)));
        assert_var(scope, "a", JSValue::number(1));

        println!("2");
        declare_const(scope ,"b", Some(JSValue::number(2)));
        assert_var(scope, "b", JSValue::number(2));

        println!("3");
        set_var(scope, "a", JSValue::number(3));
        assert_var(scope, "a", JSValue::number(3));

        println!("4");
        assert_error(scope, "b"); // error on const variable set

        println!("5");
        declare_let(scope, "c", None);
        assert_var(scope, "c", JSValue::undefined());

        println!("6");
        set_var(scope, "c", JSValue::number(4));
        assert_var(scope, "c", JSValue::number(4));

        println!("7");
        {
            let scope2 = scope.enter(vec![], vec![]);
            println!("8");
            assert_var(scope, "a", JSValue::number(3));
            declare_const(scope, "a", Some(JSValue::number(11)));
            assert_var(scope, "a", JSValue::number(11));

            println!("9");
            assert_var(scope, "b", JSValue::number(2));
            declare_let(scope, "b", Some(JSValue::number(12)));
            assert_var(scope, "b", JSValue::number(12));

            println!("10");
            assert_error(scope, "x"); // error on undeclared variable
            declare_let(scope, "x", Some(JSValue::number(13)));
            assert_var(scope, "x", JSValue::number(13));

            println!("11");
            assert_var(scope, "c", JSValue::number(4));
            set_var(scope, "c", JSValue::number(14));
            assert_var(scope, "c", JSValue::number(14));
            // scope2 drops, local scope variables automatically freed
        }; 

        println!("11");

        println!("12");
        assert_var(scope, "a", JSValue::number(3));
        assert_var(scope, "b", JSValue::number(2));
        assert_var(scope, "c", JSValue::number(14));
        assert_error(scope, "x"); // error on undeclared variable

        set_var(scope, "a", JSValue::number(5));
        assert_var(scope, "a", JSValue::number(5));
        assert_error(scope, "b"); // error on const variable set
    }
}