use kala_ast::ast::NodeF;
use kala_ast::ast;
use crate::context::InterpreterContext;
use crate::literal::Literal;
use crate::value::JSValue;

#[derive(Clone, Debug)]
pub struct InterpreterF;

// Variable is either:
// - raw variable name, e.g. "x"
// - object field internal ID, with the object internal type ID, e.g. "(x as #1).#2"
// - object method internal ID, with the object internal type ID, e.g. "(x as #1).#2()"
// - function internal ID, e.g. "#1()"
// - any other short circuiting runtime optimization

#[derive(Clone, Debug)]
pub struct Identifier {
    pub id: String,
    pub opt: Option<OptimizedIdentifier>,
}

impl From<kala_ast::ast::Identifier> for Identifier {
    fn from(ident: kala_ast::ast::Identifier) -> Self {
        Identifier {
            id: ident.name,
            opt: None,
        }
    }
}

impl Identifier {
    pub fn new(id: String) -> Self {
        Identifier {
            id,
            opt: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum OptimizedIdentifier {
    // TODO
}

#[derive(Clone, Debug)]
pub struct Statement {
    pub statement: ast::Statement<InterpreterF>,
}

impl From<ast::Statement<InterpreterF>> for Statement {
    fn from(statement: ast::Statement<InterpreterF>) -> Self {
        Statement {
            statement,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Block {
    pub block: ast::BlockStatement<InterpreterF>,
}

impl From<ast::BlockStatement<InterpreterF>> for Block {
    fn from(block: ast::BlockStatement<InterpreterF>) -> Self {
        Block {
            block,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Expression {
    pub expression: ast::Expression<InterpreterF>,
}

impl From<ast::Expression<InterpreterF>> for Expression {
    fn from(expression: ast::Expression<InterpreterF>) -> Self {
        Expression {
            expression,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub function: ast::FunctionExpression<InterpreterF>,
}

impl From<ast::FunctionExpression<InterpreterF>> for Function {
    fn from(function: ast::FunctionExpression<InterpreterF>) -> Self {
        Function {
            function,
        }
    }
}

impl InterpreterF {
    pub fn eval_expression(ctx: &mut InterpreterContext, expr: &mut <Self as NodeF>::Expression) -> Option<JSValue> {
        unimplemented!()
    }

    pub fn eval_statement(ctx: &mut InterpreterContext, stmt: &mut <Self as  NodeF>::Statement) {

    }
}

impl NodeF for InterpreterF {
    type Literal = Literal;
    type Identifier = Identifier;

    type Statement = Statement;
    type Block = Block;
    type Expression = Expression;
    type Function = Function;
}