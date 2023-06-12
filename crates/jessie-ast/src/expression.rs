use crate::{operation::*, VariableCell, Function, Record, Assignment};

// paren, function, literal, array, record, variable

// PrimaryExpr, Operator Expressions(CondExpr, BinaryExpr, UnaryExpr, CallExpr), AssignExpr
// are all collapsed into single Expr type.
// Be sure not to represent any invalid states.
#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum Expr<'a> {
    DataLiteral(&'a DataLiteral<'a>) = 0,
    Array(&'a Array<'a>) = 1,
    Record(&'a Record<'a>) = 2,
    ArrowFunc(&'a Function<'a>) = 3,
    FunctionExpr(&'a Function<'a>) = 4,
    Assignment(&'a Assignment<'a>) = 5,
    CondExpr(&'a CondExpr<'a>) = 6,
    BinaryExpr(&'a BinaryExpr<'a>) = 7,
    UnaryExpr(&'a UnaryExpr<'a>) = 8,
    CallExpr(&'a CallExpr<'a>) = 9,
    // QuasiExpr() = 10
    ParenedExpr(&'a Expr<'a>) = 11,
    Variable(&'a VariableCell<'a>) = 12,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Array<'a>(pub &'a [Element<'a>]);


#[derive(Debug, PartialEq, Clone)]
pub enum Element<'a> {
    Expr(Expr<'a>),
    Spread(Expr<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataLiteral<'a> {
    Null,
    False,
    True,
    Integer(&'a str),
    Decimal(&'a str),
    String(&'a str),
    Undefined,
    Bigint(&'a str),
}



/*
impl<'a> Expr<'a> {
    pub fn new_number(n: i64) -> Self {
        Expr::DataLiteral(DataLiteral::Integer(n.to_string()))
    }

    pub fn new_add(l: Expr, r: Expr) -> Self {
        Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Add, l, r)))
    }

    pub fn new_sub(l: Expr, r: Expr) -> Self {
        Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Sub, l, r)))
    }
}

*/


/*
#[derive(Debug, PartialEq, Clone)]
pub enum BlockOrExpr {
    Block(Vec<Statement>),
    Expr(Expr), // only appears for arrow functions
}
*/

/* 
// Function is used for function declaration, function expressions, and arrow functions.
#[derive(Debug, PartialEq, Clone)]
pub struct Function<'a>{
    pub name: Option<DefVariable>,


    pub parameters: DeclarationPointer, // must be Parameters, TODO
    pub typeann: Option<TypeAnn>,
    
    // block body
    pub statements: Vec<Statement>,

    // arrow function expression body
    pub expression: Option<Expr>,

    pub scope: Scope,

    pub captures: Option<Vec<(Field, MutableDeclarationPointer)>>, 
}

impl Function {
    pub fn from_body(name: Option<DefVariable>, parameters: DeclarationPointer, typeann: Option<TypeAnn>, block_or_expr: BlockOrExpr, scope: Scope) -> Self {
        match block_or_expr {
            BlockOrExpr::Block(statements) => Function {
                name,
                parameters,
                typeann,
                statements,
                expression: None,
                scope,
            },
            BlockOrExpr::Expr(expression) => Function {
                name,
                parameters,
                typeann,
                statements: vec![],
                expression: Some(expression),
                scope,
            },
        }
    }
}

*/

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpr<'a>(pub BinaryOp, pub Expr<'a>, pub Expr<'a>);

#[derive(Debug, PartialEq, Clone)]
pub struct CondExpr<'a>(pub Expr<'a>, pub Expr<'a>, pub Expr<'a>);

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpr<'a> {
    pub op: &'a [UnaryOp],
    pub expr: Expr<'a>,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum CallPostOp<'a> {
    Index(Expr<'a>) = 0,
    Member(&'a str) = 1,
    // QuasiExpr = 2
    Call(&'a [Arg<'a>]) = 3,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpr<'a> {
    pub expr: Expr<'a>,
    pub post_op: &'a [CallPostOp<'a>],
}

#[derive(Debug, PartialEq, Clone)]
pub enum Arg<'a> {
    Expr(Expr<'a>),
    Spread(Expr<'a>),
}
