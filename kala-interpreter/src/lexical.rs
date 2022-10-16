use kala_ast::ast::{NodeF, ParameterElement};
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

impl Statement {
    pub fn variable_declaration(kind: ast::DeclarationKind, declarators: Vec<ast::VariableDeclarator<InterpreterF>>) -> Self {
        Self {
            statement: ast::Statement::VariableDeclaration(Box::new(ast::VariableDeclaration {
                kind,
                declarators,
            }))
        }
    }

    pub fn function_declaration(id: Option<Identifier>, params: Vec<ast::Pattern<InterpreterF>>, body: Block) -> Self {
        Self {
            statement: ast::Statement::FunctionDeclaration(Box::new(ast::FunctionDeclaration {
                function: Function::new(id, params, body),
            }))
        }
    }

    pub fn block(body: Vec<Statement>) -> Self {
        Self {
            statement: ast::Statement::Block(Box::new(Block::new(body)))
        }
    }

    pub fn if_statement(test: Expression, consequent: Statement, alternate: Option<Statement>) -> Self {
        Self {
            statement: ast::Statement::If(Box::new(ast::IfStatement {
                test,
                consequent,
                alternate,
            }))
        }
    }

    pub fn while_statement(test: Expression, body: Statement) -> Self {
        Self {
            statement: ast::Statement::While(Box::new(ast::WhileStatement {
                test,
                body,
            }))
        }
    }

    pub fn break_statement() -> Self {
        Self {
            statement: ast::Statement::Break(Box::new(ast::BreakStatement {
            }))
        }
    }

    pub fn continue_statement() -> Self {
        Self {
            statement: ast::Statement::Continue(Box::new(ast::ContinueStatement {
            }))
        }
    }

    pub fn return_statement(argument: Option<Expression>) -> Self {
        Self {
            statement: ast::Statement::Return(Box::new(ast::ReturnStatement {
                argument,
            }))
        }
    }

    pub fn expression_statement(expression: Expression) -> Self {
        Self {
            statement: ast::Statement::Expression(Box::new(ast::ExpressionStatement {
                expression,
            }))
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

impl Block {
    pub fn new(body: Vec<Statement>) -> Self {
        Self {
            block: ast::BlockStatement {
                body,
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Expression {
    pub expression: ast::Expression<InterpreterF>,
}

impl Expression {
    pub fn literal(literal: Literal) -> Self {
        Self {
            expression: ast::Expression::Literal(Box::new(literal)),
        }
    }

    pub fn array(elems: Vec<Expression>) -> Self {
        let mut elements: Vec<ast::ParameterElement<InterpreterF>> = vec![];

        for elem in elems {
            elements.push(ast::ParameterElement::Parameter(elem));
        }

       Self {
            expression: ast::Expression::Array(Box::new(ast::ArrayExpression {
                elements
            })),
       } 
    }

    pub fn object(props: Vec<ast::ObjectElement<InterpreterF>>) -> Self {
        Self {
            expression: ast::Expression::Object(Box::new(ast::ObjectExpression {
                properties: props,
            })),
        }
    }

    pub fn function(name: Option<Identifier>, params: Vec<ast::Pattern<InterpreterF>>, body: Block) -> Self {
        Self {
            expression: ast::Expression::Function(Box::new(ast::FunctionExpression {
                name,
                params,
                body,
            })),
        }
    }

    pub fn arrow_function(params: Vec<ast::Pattern<InterpreterF>>, body: ast::ArrowFunctionBody<InterpreterF>) -> Self {
        Self {
            expression: ast::Expression::ArrowFunction(Box::new(ast::ArrowFunctionExpression {
                params,
                body,
            })),
        }
    }

    pub fn unary(op: ast::UnaryOperator, expr: Expression) -> Self {
        Self {
            expression: ast::Expression::Unary(Box::new(ast::UnaryExpression {
                operator: op,
                argument: expr,
            })),
        }
    }

    pub fn binary(op: ast::BinaryOperator, left: Expression, right: Expression) -> Self {
        Self {
            expression: ast::Expression::Binary(Box::new(ast::BinaryExpression {
                operator: op,
                left,
                right,
            })),
        }
    }

    pub fn logical(op: ast::LogicalOperator, left: Expression, right: Expression) -> Self {
        Self {
            expression: ast::Expression::Logical(Box::new(ast::LogicalExpression {
                operator: op,
                left,
                right,
            })),
        }
    }

    pub fn conditional(test: Expression, consequent: Expression, alternate: Expression) -> Self {
        Self {
            expression: ast::Expression::Conditional(Box::new(ast::ConditionalExpression {
                test,
                consequent,
                alternate,
            })),
        }
    }

    pub fn update(op: ast::UpdateOperator, prefix: bool, argument: Expression) -> Self {
        Self {
            expression: ast::Expression::Update(Box::new(ast::UpdateExpression {
                operator: op,
                prefix,
                argument,
            })),
        }
    }

    pub fn variable(ident: Identifier) -> Self {
        Self {
            expression: ast::Expression::Variable(Box::new(ast::VariableExpression {
                name: ident,
            })),
        }
    }

    pub fn assignment(operator: ast::AssignmentOperator, left: ast::LValue<InterpreterF>, right: Expression) -> Self {
        Self {
            expression: ast::Expression::Assignment(Box::new(ast::AssignmentExpression {
                operator,
                left,
                right,
            })),
        }
    }

    pub fn member(object: Expression, property: ast::Member<InterpreterF>) -> Self {
        Self {
            expression: ast::Expression::Member(Box::new(ast::MemberExpression {
                object,
                property,
            })),
        }
    }

    pub fn call(callee: Expression, args: Vec<Expression>) -> Self {
        let mut arguments: Vec<ast::ParameterElement<InterpreterF>> = vec![];

        for arg in args {
            arguments.push(ast::ParameterElement::Parameter(arg));
        }

        Self {
            expression: ast::Expression::Call(Box::new(ast::CallExpression {
                callee,
                arguments,
            })),
        }
    }

    pub fn parenthesized(expr: Expression) -> Self {
        Self {
            expression: ast::Expression::Parenthesized(Box::new(ast::ParenthesizedExpression {
                expression: expr,
            })),
        }
    }
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

impl Function {
    pub fn new(name: Option<Identifier>, params: Vec<ast::Pattern<InterpreterF>>, body: Block) -> Self {
        Self {
            function: ast::FunctionExpression {
                name,
                params,
                body,
            },
        }
    }
}
/* 
impl InterpreterF {
    pub fn eval_expression(
        ctx: &mut InterpreterContext, 
        eval: impl Fn(&mut InterpreterContext, ast::Expression<InterpreterF>) -> Option<JSValue>, 
        expr: &mut <Self as NodeF>::Expression
    ) -> Option<JSValue> {
        eval(ctx, expr.expression) // No additional logic for now
    }

    pub fn eval_statement(
        ctx: &mut InterpreterContext, 
        eval: impl Fn(&mut InterpreterContext, ast::Statement<InterpreterF>) -> Option<JSValue>,
        stmt: &mut <Self as NodeF>::Statement
    ) {
        eval(ctx, stmt.statement);
    }
}
*/
impl NodeF for InterpreterF {
    type Literal = Literal;
    type Identifier = Identifier;

    type Statement = Statement;
    type Block = Block;
    type Expression = Expression;
    type Function = Function;
}