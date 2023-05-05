// justin + jessie


use std::fmt::Debug;


// use crate::{jessie_scope::{Scope, Variable}};
use crate::jessie_operation::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Array(pub Vec<Element>);
/* 
impl From<json_types::Array> for Array {
    fn from(arr: json_types::Array) -> Self {
        Array(arr.0.into_iter().map(|e| e.into()).collect())
    }
}
*/

#[derive(Debug, PartialEq, Clone)]
pub struct Record(pub Vec<PropDef>);
/* 
impl From<json_types::Record> for Record {
    fn from(rec: json_types::Record) -> Self {
        Record(rec.0.into_iter().map(|e| e.into()).collect())
    }
}
*/

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Expr(Expr),
    Spread(Expr),
}
/* 
impl From<json_types::Element> for Element {
    fn from(el: json_types::Element) -> Self {
        match el {
            json_types::Element::Expr(e) => Element::Expr(e.into()),
        }
    }
}
*/

#[derive(Debug, PartialEq, Clone)]
pub enum DataLiteral {
    Null,
    False,
    True,
    Number(String),
    String(String),
    Undefined,
    Bigint(String),
}
/*
impl From<json_types::DataLiteral> for DataLiteral {
    fn from(lit: json_types::DataLiteral) -> Self {
        match lit {
            json_types::DataLiteral::Null => DataLiteral::Null,
            json_types::DataLiteral::False => DataLiteral::False,
            json_types::DataLiteral::True => DataLiteral::True,
            json_types::DataLiteral::Number(n) => DataLiteral::Number(n),
            json_types::DataLiteral::String(s) => DataLiteral::String(s),
        }
    }
}
*/

#[derive(Debug, PartialEq, Clone)]
pub enum PropDef {
    KeyValue(PropName, Expr), 
    // MethodDef(MethodDef),// TODO
    Shorthand(PropName),
    Spread(Expr),
}

/*
impl From<json_types::PropDef> for PropDef {
    fn from(pd: json_types::PropDef) -> Self {
        match pd {
            json_types::PropDef::KeyValue(k, v) => PropDef::KeyValue(k.into(), v.into()),
        }
    }
}
*/

#[derive(Debug, PartialEq, Clone)]
pub enum TypeAnn{}

#[derive(Debug, PartialEq, Clone)]
pub enum PropName {
    String(String),
    Number(String),
    Ident(String),
}
/*
impl From<json_types::PropName> for PropName {
    fn from(pn: json_types::PropName) -> Self {
        match pn {
            json_types::PropName::String(s) => PropName::String(s),
            json_types::PropName::Number(n) => PropName::Number(n),
            json_types::PropName::Ident(i) => PropName::Ident(i),
        }
    }
}
*/








#[derive(Debug, PartialEq, Clone)]
pub struct ModuleBody(pub Vec<ModuleItem>);

#[derive(Debug, PartialEq, Clone)]
pub enum ModuleItem {
    // ImportDecl(ImportDecl),
    // ExportDecl(ExportDecl),
    ModuleDecl(ModuleDecl)
}

#[derive(Debug, PartialEq, Clone)]
pub struct ModuleDecl(pub Vec<ModuleBinding>);


/* 
#[derive(Debug, PartialEq, Clone)]
pub struct HardenedExpr(pub Expr); // TODO
*/


#[derive(Debug, PartialEq, Clone)]
pub enum ModuleBinding {
    VariableBinding(String, Option</*Hardened*/Expr>),
    PatternBinding(/*Binding*/Pattern, /*Hardened*/Expr),
}



// BindingPattern, Param, Pattern are all collapsed into single Pattern type
// Be careful to not mess with parsing orders - struct types and parsing might not correspond
#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    Rest(Box<Pattern>, Option<TypeAnn>),
    Optional(String, Box<Expr>, Option<TypeAnn>),
    ArrayPattern(Vec<Pattern>, Option<TypeAnn>), // only Vec<Param> form is valid
    RecordPattern(Vec<PropParam>, Option<TypeAnn>),
    Variable(String, Option<TypeAnn>),
    // DataLiteral(DataLiteral), // I don't understand why dataliteral is here...
    Hole,
}

impl Pattern {
    pub fn rest(pattern: Pattern) -> Pattern {
        Pattern::Rest(Box::new(pattern), None)
    }

    pub fn optional(name: String, expr: Expr) -> Pattern {
        Pattern::Optional(name, Box::new(expr), None)
    }

    pub fn array(patterns: Vec<Pattern>) -> Pattern {
        Pattern::ArrayPattern(patterns, None)
    }

    pub fn record(props: Vec<PropParam>) -> Pattern {
        Pattern::RecordPattern(props, None)
    }

    pub fn variable(name: String) -> Pattern {
        Pattern::Variable(name, None)
    }
}

impl From<Expr> for Pattern {
    fn from(value: Expr) -> Self {
        // Expression can be converted to pattern only if it is 
        // - a variable
        // - an assignment to a variable
        // - array compatible with destructuring
        // - object compatible with destructuring
        match value {
            Expr::Variable(name) => Pattern::Variable(name, None),
            Expr::Assignment(assign) => unimplemented!("optional"),
            Expr::Array(arr) => unimplemented!("array"),
            Expr::Record(rec) => unimplemented!("record"), 
            _ => panic!("Cannot convert expr to pattern"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PropParam {
    Rest(Pattern),
    KeyValue(String, Pattern),
    Optional(String, Expr),
    Shorthand(String),
}

// paren, function, literal, array, record, variable

// PrimaryExpr, Operator Expressions(CondExpr, BinaryExpr, UnaryExpr, CallExpr), AssignExpr
// are all collapsed into single Expr type.
// Be sure not to represent any invalid states.
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    DataLiteral(DataLiteral),
    Array(Array),
    Record(Record),
    ArrowFunc(Box<Function>),
    FunctionExpr(Box<Function>),
    Assignment(Box<Assignment>),
    CondExpr(Box<CondExpr>),
    BinaryExpr(Box<BinaryExpr>),
    UnaryExpr(Box<UnaryExpr>),
    CallExpr(Box<CallExpr>),
    // QuasiExpr()
    ParenedExpr(Box<Expr>),
    Variable(String),
}

impl Expr {
    pub fn try_into_lvalue(self) -> Option<LValue> {
        unimplemented!()
    }
}

/* 
impl From<json_types::DataStructure> for Expr {
    fn from(ds: json_types::DataStructure) -> Self {
        match ds {
            json_types::DataStructure::DataLiteral(lit) => Expr::DataLiteral(lit.into()),
            json_types::DataStructure::Array(arr) => Expr::Array(arr.into()),
            json_types::DataStructure::Record(rec) => Expr::Record(rec.into()),
        }
    }
}
*/

impl From<LValue> for Expr {
    fn from(lv: LValue) -> Self {
        match lv {
            LValue::Variable(v) => Expr::Variable(v),
            LValue::Index(expr, index) => Expr::CallExpr(Box::new(CallExpr{expr, post_op: CallPostOp::MemberPostOp(MemberPostOp::Index(index))})),
            LValue::Member(expr, member) => Expr::CallExpr(Box::new(CallExpr{expr, post_op: CallPostOp::MemberPostOp(MemberPostOp::Member(member))})),
        }
    }
}

impl Expr {
    pub fn new_number(n: i64) -> Self {
        Expr::DataLiteral(DataLiteral::Number(n.to_string()))
    }

    pub fn new_add(l: Expr, r: Expr) -> Self {
        Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Add, l, r)))
    }

    pub fn new_sub(l: Expr, r: Expr) -> Self {
        Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Sub, l, r)))
    }
}



#[derive(Debug, PartialEq, Clone)]
pub struct Assignment(pub LValue, pub AssignOp, pub Expr);

#[derive(Debug, PartialEq, Clone)]
pub enum AssignOp {
    Assign,
    AssignAdd,
    AssignSub,
    AssignMul,
    AssignDiv,
    AssignMod,
    AssignExp,
    AssignLShift,
    AssignRShift,
    AssignURShift,
    AssignBitAnd,
    AssignBitXor,
    AssignBitOr,
}



/*
#[derive(Debug, PartialEq, Clone)]
pub enum PureExpr {
    ArrowFunc(Function),
    Parent(json::PureExpr),
    ParenedExpr(Box<PureExpr>),
    Variable(String),
}
*/

#[derive(Debug, PartialEq, Clone)]
pub enum LValue {
    Index(Expr, Expr),
    Member(Expr, String),
    Variable(String),
}

impl From<Expr> for LValue {
    fn from(value: Expr) -> Self {
        match value {
            Expr::Variable(name) => LValue::Variable(name),
            Expr::CallExpr(call) => {
                match call.post_op {
                    CallPostOp::MemberPostOp(MemberPostOp::Index(index)) => LValue::Index(call.expr, index),
                    CallPostOp::MemberPostOp(MemberPostOp::Member(member)) => LValue::Member(call.expr, member),
                    _ => panic!("Invalid LValue"),
                }
            },
            _ => panic!("Invalid LValue"),
        }
    }
}

// StatementItem in Jessie
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Declaration(Declaration),
    FunctionDeclaration(Function),
    Block(Block),
    IfStatement(IfStatement),
    // ForStatement(ForStatement),
    WhileStatement(WhileStatement),
    Continue,
    Break,
    Return(Option<Expr>),
    Throw(Expr),
    // TryStatement(TryStatement),
    ExprStatement(Expr),
}






#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    //pub local_scope: Scope,

    pub statements: Vec<Statement>,
    pub declarations: Vec<(String, DeclarationKind)>,
    pub uses: Vec<String>,
}

impl Block {
    pub fn new(statements: Vec<Statement>, declarations: Vec<(String, DeclarationKind)>, uses: Vec<String>) -> Self {
        Block {
     //       local_scope: Scope::empty(), // TODO 
            statements,
            declarations,
            uses,
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement {
    pub condition: Expr,
    pub consequent: Block,
    pub alternate: Option<ElseArm>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ElseArm {
    Body(Block),
    ElseIf(Box<IfStatement>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileStatement {
    pub condition: Expr,
    pub body: Block,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StatementItem {
    Declaration(Declaration),
    FunctionDecl(Function),
    Statement(Statement),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DeclarationKind {
    Let,
    Const,
    Argument,
    Function,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Declaration {
    pub kind: DeclarationKind,
    pub bindings: Vec<Binding>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Binding {
    VariableBinding(String, Option<Expr>),
    PatternBinding(/*Binding*/Pattern, Expr),
}
/*
#[derive(Debug, PartialEq, Clone)]
pub enum PurePropDef {
    // MethodDef(MethodDef),//TODO
    Parent(json::PurePropDef),
    Shorthand(PropName),
    Spread(Expr),
}
*/

#[derive(Debug, PartialEq, Clone)]
pub enum BlockOrExpr {
    Block(Vec<Statement>),
    Expr(Expr), // only appears for arrow functions
}

// Function is used for function declaration, function expressions, and arrow functions.
#[derive(Debug, PartialEq, Clone)]
pub struct Function{
    pub name: Option<String>,
    pub parameters: Vec<Pattern/*Param*/>,
    pub typeann: Option<TypeAnn>,
    
    // block body
    pub statements: Vec<Statement>,

    // arrow function expression body
    pub expression: Option<Expr>,

    pub declarations: Vec<(String, DeclarationKind)>,
    pub uses: Vec<String>,
}

impl Function {
    pub fn from_body(name: Option<String>, parameters: Vec<Pattern>, typeann: Option<TypeAnn>, block_or_expr: BlockOrExpr, declarations: Vec<(String, DeclarationKind)>, uses: Vec<String>) -> Self {
        match block_or_expr {
            BlockOrExpr::Block(statements) => Function {
                name,
                parameters,
                typeann,
                statements,
                expression: None,
                declarations,
                uses,
            },
            BlockOrExpr::Expr(expression) => Function {
                name,
                parameters,
                typeann,
                statements: vec![],
                expression: Some(expression),
                declarations,
                uses,
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpr(pub BinaryOp, pub Expr, pub Expr);

#[derive(Debug, PartialEq, Clone)]
pub struct CondExpr(pub Expr, pub Expr, pub Expr);

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpr {
    pub op: Vec<UnaryOp>,
    pub expr: Expr,
}
#[derive(Debug, PartialEq, Clone)]
pub enum CallPostOp {
    MemberPostOp(MemberPostOp),
    Call(Vec<Arg>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpr {
    pub expr: Expr,
    pub post_op: CallPostOp,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MemberPostOp {
    Index(Expr),
    Member(String),
    // QuasiExpr
}

#[derive(Debug, PartialEq, Clone)]
pub enum Arg {
    Expr(Expr),
    Spread(Expr),
}

