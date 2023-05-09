// justin + jessie


use std::cell::{RefCell, OnceCell};
use std::fmt::Debug;
use std::rc::Rc;


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
    Integer(String),
    Decimal(String),
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
    Shorthand(UseVariable),
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
    VariableBinding(DefVariable, Option</*Hardened*/Expr>),
    PatternBinding(/*Binding*/Pattern, /*Hardened*/Expr),
}



// BindingPattern, Param, Pattern are all collapsed into single Pattern type
// Be careful to not mess with parsing orders - struct types and parsing might not correspond
#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    Rest(Box<Pattern>, Option<TypeAnn>),
    Optional(DefVariable, Box<Expr>, Option<TypeAnn>),
    ArrayPattern(Vec<Pattern>, Option<TypeAnn>), // only Vec<Param> form is valid
    RecordPattern(Vec<PropParam>, Option<TypeAnn>),
    Variable(DefVariable, Option<TypeAnn>),
    // DataLiteral(DataLiteral), // I don't understand why dataliteral is here...
    Hole,
}

impl Pattern {
    pub fn rest(pattern: Pattern) -> Pattern {
        Pattern::Rest(Box::new(pattern), None)
    }

    pub fn optional(name: DefVariable, expr: Expr) -> Pattern {
        Pattern::Optional(name, Box::new(expr), None)
    }

    pub fn array(patterns: Vec<Pattern>) -> Pattern {
        Pattern::ArrayPattern(patterns, None)
    }

    pub fn record(props: Vec<PropParam>) -> Pattern {
        Pattern::RecordPattern(props, None)
    }

    pub fn variable(name: DefVariable) -> Pattern {
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
            Expr::Variable(name) => Pattern::Variable(name.into(), None),
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
    KeyValue(DefVariable, Pattern),
    Optional(DefVariable, Expr),
    Shorthand(DefVariable),
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
    Variable(UseVariable),
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
        Expr::DataLiteral(DataLiteral::Integer(n.to_string()))
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

// TODO: LValue should support nested index/member
#[derive(Debug, PartialEq, Clone)]
pub enum LValue {
    Index(Expr, Expr),
    Member(Expr, Field),
    Variable(UseVariable),
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
    Declaration(DeclarationPointer),
    Block(Box<Block>),
    IfStatement(Box<IfStatement>),
    // ForStatement(ForStatement),
    WhileStatement(Box<WhileStatement>),
    Continue,
    Break,
    Return(Option<Expr>),
    Throw(Expr),
    // TryStatement(TryStatement),
    ExprStatement(Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Scope {
    pub declarations: Option<Box<Vec<DeclarationPointer>>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DefVariable{
    pub name: String,
    // pub decl: MutableDeclarationPointer, // DefVariable always occures within a declaration anyway
}

impl From<String> for DefVariable {
    fn from(name: String) -> Self {
        DefVariable {
            name,
        }
    }
}

impl<'a> From<&'a str> for DefVariable {
    fn from(name: &'a str) -> Self {
        DefVariable {
            name: name.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UseVariable {
    pub name: String,
    pub decl: MutableDeclarationPointer,
}

impl UseVariable {
    pub fn new(name: &'static str, decl: DeclarationPointer) -> Self {
        UseVariable {
            name: name.to_string(),
            decl: MutableDeclarationPointer(Rc::new(RefCell::new(Rc::new(OnceCell::from(decl))))),
        }
    }

    pub fn new_unbound(name: &'static str) -> Self {
        UseVariable {
            name: name.to_string(),
            decl: MutableDeclarationPointer(Rc::new(RefCell::new(Rc::new(OnceCell::new())))),
        }
    }
}

impl Into<DefVariable> for UseVariable {
    fn into(self) -> DefVariable {
        DefVariable {
            name: self.name,
        }
    }
}

// Mutable declaration pointer is a double pointer to declaration.
// The struct represents three steps of indirection:
// 1. declarations too larged to be cloned, so the scope information holds the shared reference to it(Rc<Declaration>)
// 2. when a variable is not yet bind to a specific declaration(variable from a parent scope, etc), it should be first initialized as a None. After then, the variable should be shared by multiple ast Variable nodes, and also should be passed to the parent variable, where the parent can set the reference to the declaration if found. All the occurance in the child block will be also overriden at this point.
// 3. when a variable is used in multiple level of block scopes(with all under the same declaration), they should be all pointing to the same pointer that points a spceific declaration slot(initialized to None). 
#[derive(Debug, PartialEq, Clone)]
pub struct MutableDeclarationPointer(Rc<RefCell<Rc<OnceCell<DeclarationPointer>>>>); // &mut &mut Option<&Declaration>

impl MutableDeclarationPointer {
    pub fn new() -> Self {
        MutableDeclarationPointer(Rc::new(RefCell::new(Rc::new(OnceCell::new()))))
    }

    pub fn get(&self) -> Option<DeclarationPointer> {
        (*self.0).borrow().get().cloned()
    }

    // used in once-set context in case 2
    pub fn settle(&mut self, var: DeclarationPointer) {
        self.0.borrow_mut().set(var).unwrap();
    }

    // used in parent propagation context in case 3
    // existing variable should be not set
    pub fn replace(&mut self, var: MutableDeclarationPointer) {
        let self_cell: &mut Rc<OnceCell<DeclarationPointer>> = &mut(*self.0).borrow_mut();
        if self_cell.get().is_some() {
            unreachable!("Variable already locally set, should not be replaced");
        }
        let var_cell: &Rc<OnceCell<DeclarationPointer>> = &(*var.0).borrow();
        *self_cell = var_cell.clone();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    //pub local_scope: Scope,

    pub statements: Vec<Statement>,
    pub scope: Scope,
}

impl Block {
    pub fn new(statements: Vec<Statement>, scope: Scope) -> Self {
        Block {
     //       local_scope: Scope::empty(), // TODO 
            statements,
            scope,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DeclarationKind {
    Let,
    Const,
    Argument,
    Function,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    Let(Vec<Binding>),
    Const(Vec<Binding>),
    Function(Box<Function>),
    Parameters(Vec<Pattern>),
}

pub type DeclarationPointer = Rc<Declaration>;

#[derive(Debug, PartialEq, Clone)]
pub enum Binding {
    VariableBinding(DefVariable, Option<Expr>),
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
    Member(Field),
    // QuasiExpr
}

#[derive(Debug, PartialEq, Clone)]
pub enum Arg {
    Expr(Expr),
    Spread(Expr),
}

