/*
Executions are defined as following forms:

- Assignment of Value, Object Literals, Function call results
- Arithmetic expressions between Values and variables
- Control expressions, namely loop and conditionals

- short circuited operations are translated into control expressions
*/

struct AssignVar {
    varname: String
    properties: Vec<String>
}

impl AssignVar {

    fn assign_literal_null(&mut self) {

    }

    fn assign_literal_number(&mut self, value: i64) {

    }

    fn assign_literal_boolean(&mut self, value: bool) {

    }

    fn assign_literal_object(&mut self, value: ObjectLiteral) {

    }

    fn assign_literal_array(&mut self, value: ArrayLiteral) {

    }

    fn assign_literal_undefined(&mut self) {

    }

    fn assign_var_shared(&mut self, value: Rc<Value>) {

    }

    fn assign_var_moved(&mut self, value: Value) {

    }

    fn assign_fn_result(&mut self, value: Value) {

    }
}

enum ArithmeticVar {
    Lit(DataLiteral),
    Var(String),
}

struct ArithmeticExpr {
    
}

