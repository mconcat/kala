// UNUSED 
// TODO: move to codegen context

use kala_ast::ast;
use kala_context::{environment_record::{EnvironmentRecord}};

use crate::{value::JSValue, lexical::InterpreterF, eval::eval_statement};
use crate::prototype::Prototype;

#[cfg(test)]
mod reference_test {
    use crate::value::JSObject;
    use crate::value::JSValue;

    #[test]
    fn simple_object_test() {
        fn get(obj: &JSValue, name: &str) -> JSValue {
            obj.get_property(JSValue::string(name.to_string())).unwrap().clone()
        }

        fn set(obj: &mut JSValue, name: &str, value: JSValue) {
            obj.set_property(&JSValue::string(name.to_string()), value)
        }

        let mut obj = JSValue::object(vec![
            ("a".to_string(), JSValue::number(1)),
            ("b".to_string(), JSValue::number(2)),
            ("c".to_string(), JSValue::number(3)),
        ]);

        assert_eq!(get(&obj, "a"), JSValue::number(1));

        assert_eq!(get(&obj, "b"), JSValue::number(2));
        
        assert_eq!(get(&obj, "c"), JSValue::number(3));

        set(&mut obj, "a", JSValue::number(4));
        assert_eq!(get(&obj, "a"), JSValue::number(4));

        set(&mut obj, "b", JSObject::new(vec![(JSValue::string("x".to_string()), JSValue::number(5))]));
        assert_eq!(get(&get(&obj, "b"), "x"), JSValue::number(5));
    }
}



enum CompletionRecord {
    Normal,
    Return(Option<JSValue>),
    Throw(JSValue),
    Break,
    Continue,
}

pub struct JSContext {
    environment_record: EnvironmentRecord<JSValue>,
    completion: CompletionRecord,
}

impl JSContext {
    fn with_env(&mut self, env: EnvironmentRecord<JSValue>, f: impl Fn(&mut Self)) {
        let current_context = JSContext {
            environment_record: env,
            completion: CompletionRecord::Normal,
        };

        f(&mut current_context);

        self.completion = current_context.completion;

        // env drops here
    }
}

impl JSContext {
    fn undefined() -> JSValue {
        JSValue::undefined()
    }

    fn null() -> JSValue {
        JSValue::null()
    }

    fn boolean(b: bool) -> JSValue {
        JSValue::boolean(b)
    }

    fn coerce_boolean(&mut self, v: &JSValue) -> JSValue {
        match v {
            JSValue::Boolean(b) => JSValue::boolean(b.0),
            _ => unimplemented!("coerce boolean"),
        }
    }

    fn number(n: i64) -> JSValue {
        JSValue::number(n)
    }

    fn coerce_number(&mut self, v: &JSValue) -> JSValue {
        match v {
            JSValue::Number(n) => v.clone(),
            _ => unimplemented!("coerce number"),
        }
    }

    fn bigint(n: i64) -> JSValue {
        JSValue::bigint(n)
    }

    fn string(s: String) -> JSValue {
        JSValue::string(s)
    }

    fn coerce_string(&mut self, v: &JSValue) -> JSValue {
        match v {
            JSValue::String(s) => s.clone(),
            _ => unimplemented!("coerce string"),
        }
    }

    fn array(&mut self, v: Vec<JSValue>) -> JSValue {
        JSValue::array(v)
    }

    fn object(&mut self, props: Vec<(String, JSValue)>) -> JSValue {
        JSValue::object(props)
    }

    fn function(&mut self, captures: Vec<String>, code: ast::FunctionExpression<InterpreterF>) -> JSValue {
        let mut captured_vars = Vec::with_capacity(captures.len());
        
        let scope = self.environment_record.function_scope(captures);


        JSValue::function(captured_vars, code)
    }

    fn block_scope(&mut self, hoisted_funcs: Vec<(String, JSValue)>, body: impl Fn(&mut Self)) {
        self.environment_record.enter();
        for (name, func) in hoisted_funcs {
            self.initialize_immutable_binding(&name, &func)
        }  
        body(self);
        self.environment_record.exit()
    }

    fn call(&mut self, eval: impl Fn(&mut Self, ast::Statement<InterpreterF>) -> JSValue, func: &JSValue, args: Vec<JSValue>) -> Result<(), String> {
        if let JSValue::Object(ptr) = func {
            let mut obj = ptr.borrow_mut();
            if let Some(Prototype::Function(env, f)) = obj.prototype {
                let params = args.iter().enumerate().map(|(i, arg)| (f.parameters[i], arg));
                let local_env = env.enter(params);
                self.with_env(local_env, |ctx| { eval(ctx, &f); });
                return Ok(())
            } 
        }

        Err("cannot call non function".to_string())
    }

    #[inline]
    fn control_loop(&mut self, test: impl Fn(&mut Self) -> JSValue, body: impl Fn(&mut Self)) {
        while test(self).is_truthy() {
            body(self);
        }
    }

    #[inline]
    fn control_branch(&mut self, test: impl Fn(&mut Self) -> JSValue, consequent: impl Fn(&mut Self), alternate: impl Fn(&mut Self)) {
        if test(self).is_truthy() {
            consequent(self);
        } else {
            alternate(self);
        }
    }

    #[inline]
    fn control_branch_value(&mut self, test: impl Fn(&mut Self) -> JSValue, consequent: impl Fn(&mut Self) -> JSValue, alternate: impl Fn(&mut Self) -> JSValue) -> JSValue {
        if test(self).is_truthy() {
            consequent(self)
        } else {
            alternate(self)
        }
    }

     #[inline]
    fn control_switch(&mut self) {
        unimplemented!()
    }

    #[inline]
    fn control_coalesce(&mut self, left: impl Fn(&mut Self) -> JSValue, right: impl Fn(&mut Self) -> JSValue) -> JSValue {
        let result = left(self);
        match result {
            JSValue::Undefined => right(self),
            JSValue::Null => right(self),
            _ => result,
        }
    }

    #[inline]
    fn complete_break(&mut self) {
        self.completion = CompletionRecord::Break;
    }

    fn is_break(&self) -> bool {
        match self.completion {
            CompletionRecord::Break => true,
            _ => false,
        }
    }

    #[inline]
    fn complete_continue(&mut self) {
        self.completion = CompletionRecord::Continue;
    }

    fn is_continue(&self) -> bool {
        match self.completion {
            CompletionRecord::Continue => true,
            _ => false,
        }
    }

    fn complete_return(&mut self) {
        self.completion = CompletionRecord::Return(None);
    }

    #[inline]
    fn complete_return_value(&mut self, value: JSValue) {
        self.completion = CompletionRecord::Return(Some(value));
    }

    fn is_return(&self) -> bool {
        match self.completion {
            CompletionRecord::Return(_) => true,
            _ => false,
        }
    }

    fn consume_return(&mut self) -> Option<JSValue> {
        let result = match self.completion {
            CompletionRecord::Return(Some(value)) => Some(value),
            _ => None,
        };
        self.completion = CompletionRecord::Normal;
        result
    }

    fn complete_throw(&mut self, val: JSValue) {
        self.completion = CompletionRecord::Throw(val);
    }

    fn is_throw(&self) -> bool {
        match self.completion {
            CompletionRecord::Throw(_) => true,
            _ => false,
        }
    }

    fn consume_throw(&mut self) -> JSValue {
        let result = match self.completion {
            CompletionRecord::Throw(value) => value,
            _ => unreachable!(),
        };
        self.completion = CompletionRecord::Normal;
        result
    }

    fn initialize_mutable_binding(&mut self, name: &String, v: Option<JSValue>) -> Result<(), String> {
        self.environment_record.initialize_mutable_binding(name, v);
        Ok(())
    }

    fn initialize_immutable_binding(&mut self, name: &String, v: &JSValue) -> Result<(), String> {
        self.environment_record.initialize_immutable_binding(name, v)
    }

    fn resolve_binding(&mut self, name: &String) -> JSValue {
        self.environment_record.resolve_binding(name)
    }

    fn set_binding(&mut self, name: &String, v: JSValue) -> Result<(), String> {
        self.environment_record.set_mutable_binding(name, v)
    }

    // operations

    fn op_add(&mut self, left: &mut JSValue, other: &JSValue) -> &mut JSValue {
        match (left, other) {
            (JSValue::String(l), JSValue::String(r)) => l.op_concat(r),
            (JSValue::String(_), _) | (_, JSValue::String(_)) => unimplemented!("string coersion concat"),
            (l, r) => self.coerce_number(l).op_add(self.coerce_number(r)),
        }
    }

    fn op_sub(&mut self, left: &mut JSValue, other: &JSValue) -> &mut JSValue {
        self.coerce_number(left).as_number().and_then(|left| 
        self.coerce_number(other).as_number().and_then(|right|
        left.op_sub(right))).unwrap_or_else(JSValue::type_error)
    }
}