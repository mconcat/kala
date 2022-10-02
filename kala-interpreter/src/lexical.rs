use ast::NodeF;

pub struct InterpreterF;

// Variable is either:
// - raw variable name, e.g. "x"
// - object field internal ID, with the object internal type ID, e.g. "(x as #1).#2"
// - object method internal ID, with the object internal type ID, e.g. "(x as #1).#2()"
// - function internal ID, e.g. "#1()"
// - any other short circuiting runtime optimization
pub struct Identifier {
    pub id: String,
    pub opt: Option<OptimizedIdentifier>,
}

pub enum OptimizedIdentifier {
    // TODO
}

impl ast::NodeF<Self> for InterpreterF {
    type Identifier = Identifier;
    type Variable = Variable;

    type Statement = ast::Statement<Self>;
    type Block = ast::BlockStatement<Self>;
    type Expression = ast::Expression<Self>;
    type Function = ast::FunctionExpression<Self>;
}