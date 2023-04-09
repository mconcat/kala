/*
EnvironmentRecord provides a mapping from a variable name to the variable
*/

use std::fmt::Display;
use std::hash::Hash;

use crate::evaluation_context::JSValue;
use crate::evaluation_context::JSVariable;

use core::fmt::Debug;

use std::rc::Rc;
use std::cell::RefCell;

/* 
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
    pub fn capture(&mut self) -> Self {
        // capture applies only on the closure capture and should be distinguished with heap allocation
        // (all objects are heap allocated by default, regardless of their escaping)
        // as capturing applies to the variable, not the value, we don't need to recursively
        // capture the inner references.
        match self {
            Self::Local(value, mutable) => {
                *self = Self::Captured(
                    Rc::new(RefCell::new(value.clone())),
                    *mutable,
                );
                self.capture()
            },
            Self::Captured(value, mutable) => {
                Self::Captured(Rc::clone(value), *mutable)
            }
        }
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

    pub fn as_mutable(&mut self) -> Option<&mut Self> {
        if self.is_mutable() {
            Some(self)
        } else {
            None
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
*/
use hashbrown::HashMap;

// Binding: variables that are visible in the current scope.
// Recovery: shadowed parent scope variables
#[derive(Debug, Clone)]
pub struct EnvironmentRecord<K: Eq + Hash + Clone + Debug, V: JSValue> {
    global_binding: Rc<RefCell<HashMap<K, V::Variable>>>,

    // binding is a map from variable name to its ScopedVariable
    // the binding should hold the ScopedVariables in perspective of the innermost scope.
    binding: HashMap<K, (V::Variable, usize)>,
    recovery: Vec<Vec<(K, Option<(V::Variable, usize)>)>>, // TODO: linearlize
}

impl<K: Eq + Hash + Clone + Debug, V: JSValue> EnvironmentRecord<K, V> {
    pub fn new(global_binding: Rc<RefCell<HashMap<K, <V as JSValue>::Variable>>>) -> Self {
        EnvironmentRecord {
            global_binding,
            
            binding: HashMap::new(),
            recovery: vec![vec![]],
        }
    }

    pub fn empty_global_binding() -> Rc<RefCell<HashMap<K, <V as JSValue>::Variable>>> {
        Rc::new(RefCell::new(HashMap::new()))
    }

    // enters a block scope
    pub fn enter(&mut self) {
        self.recovery.push(vec![]) 
    }

    pub fn exit(&mut self) {
        let mut current_scope = self.recovery.pop().unwrap();

        // recover the shadowed variables
        for (name, recovery) in current_scope {
            match recovery {
                Some(var) => {
                    self.binding.insert(name, var);
                },
                None => {
                    self.binding.remove(&name);
                }
            }
        }
    }
    
    // creates a new scope for function closure with provided capture variables
    pub fn closure(&mut self, captures: Vec<K>) -> Option<Self> {
        let mut env = EnvironmentRecord::new(Rc::clone(&self.global_binding));
        for capture in captures {
            if let Some((var, depth)) = self.binding.get_mut(&capture) {
                var.capture();
                env.binding.insert(capture, (var.clone(), 0));
            } else {
                return None // invalid capture
            }
        }
        Some(env)
    }

    pub fn closure_with_global_capture(&mut self) -> Self {
        let mut env = EnvironmentRecord::new(Rc::clone(&self.global_binding));
    
        // super inefficient, capture only required variables later
        for (k, (var, depth)) in self.binding.iter() {
            var.capture();
            env.binding.insert(k.clone(), (var.clone(), 0));
        }

        env
    }

    #[inline]
    pub fn depth(&self) -> usize {
        self.recovery.len()
    }

    fn add_recover_variable(&mut self, name: K, value: Option<(V::Variable, usize)>) {
        self.recovery.last_mut().unwrap().push((name, value));
    }


    #[inline]
    fn resolve_binding(&self, name: &K) -> Result<&V::Variable, String> {
        self.binding.get(name).map(|(var, _)| var).or_else(|| self.global_binding.borrow().get(name)).ok_or_else(|| format!("ReferenceError: {:?} is not defined", name))
    }

    // variable_mut should be de
    #[inline]
    fn resolve_mutable_binding(&self, name: &K) -> Result<&mut V::Variable, String> {
        self.resolve_binding(name).and_then(|var| var.as_mutable().ok_or_else(|| format!("Cannot assign to constant variable {:?}", name)))
    }

    /*
    pub fn get_immutable_binding(&self, name: &K) -> Result<V, String> {
        self.resolve_binding(name).map(|var| var.get().clone())
    }

    pub fn get_mutable_binding(&self, name: &K) -> Result<&mut V, String> {
        self.resolve_mutable_binding(name).map(|var| var.get())
    }
    */
   
    fn initialize_binding(&mut self, name: &K, value: Option<V>, mutable: bool) -> Result<(), String> {
        let existing = self.binding.get_mut(name).cloned();
        match existing {
            Some((var, depth)) => {
                // redeclaration of local binding is not allowed
                if depth == self.depth() {
                    return Err(format!("SyntaxError: redeclaration of formal parameter \"{:?}\"", name))
                }
                // add shadowing variable to recovery
                self.add_recover_variable(name.clone(), Some((var, depth)));
                self.binding.insert(name.clone(), (V::Variable::new(value.unwrap_or_default(), mutable), self.depth()));
            }
            // declaration of new variable should be discarded after the scope, add to recovery
            None => {
                self.add_recover_variable(name.clone(), None);
                self.binding.insert(name.clone(), (V::Variable::new(value.unwrap_or_default(), mutable), self.depth()));
            }
        }

        Ok(())
    }

    pub fn initialize_mutable_binding(&mut self, name: &K, value: Option<V>) -> Result<(), String> {
        self.initialize_binding(name, value, true)
    }

    pub fn initialize_immutable_binding(&mut self, name: &K, value: V) -> Result<(), String> {
        self.initialize_binding(name, Some(value), false)
    }

    pub fn set_mutable_binding(&mut self, name: &K, value: V) -> Result<(), String> {
        self.resolve_binding(name).map(|var| var.set(value))
    }
/*
    pub fn declare(&mut self, name: &String, kind: ast::DeclarationKind, value: Option<JSValue>) -> Result<(), String> {
        if kind == ast::DeclarationKind::Const && value.is_none() {
            panic!("const variable must be initialized");
        }
        
        let binding = self.binding.get(name).clone();

        // if there is no variable existing already, create a new one, and add it to the recovery list as to be discarded
        if binding.is_none() {
            self.add_recover_variable(name.clone(), None);
            let var = ScopedVariable::new(value.unwrap_or(JSValue::Undefined), kind, self.depth());
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
            return Err(format!("SyntaxError: redeclaration of formal parameter \"{}\"", name))
        }

        let var = ScopedVariable::new(value.unwrap_or(JSValue::Undefined), kind, self.depth());
        self.binding.insert(name.clone(), var);
        Ok(())
    }
*/

}
#[cfg(test)]
mod scope_tests {
    use crate::environment_record::EnvironmentRecord;
    use crate::mock::JSValue;
    use crate::evaluation_context::JSVariable;

    #[test]
    fn scope_test_simple() {
        let scope = &mut EnvironmentRecord::new(EnvironmentRecord::empty_global_binding());

        let declare_let = |scope: &mut EnvironmentRecord<String, JSValue>, name: &str, value: Option<JSValue>| {
            scope.initialize_mutable_binding(&name.to_string(), value);
        };

        let declare_const = |scope: &mut EnvironmentRecord<String, JSValue>, name: &str, value: JSValue| {
            scope.initialize_immutable_binding(&name.to_string(), value);
        };

        let set_var = |scope: &mut EnvironmentRecord<String, JSValue>, name: &str, value: JSValue| {
            scope.resolve_binding(&name.to_string()).map(|v| v.set(value)).unwrap()
        };

        let assert_set_error = |scope: &mut EnvironmentRecord<String, JSValue>, name: &str| {
            assert!(scope.resolve_binding(&name.to_string()).unwrap().is_mutable() == false)
        };

        let assert_var = |scope: &mut EnvironmentRecord<String, JSValue>, name: &str, value: JSValue| {
            assert_eq!(scope.resolve_binding(&name.to_string()).unwrap().get(), value);
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
        scope.enter();
        {
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
        }; 

        println!("11");
        scope.exit();

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