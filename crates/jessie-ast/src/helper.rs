use utils::{SharedString};

use crate::{*};
pub use crate::traits::UnsafeInto;

// Expression

// Data Literal

fn data_literal(data_literal: DataLiteral) -> Expr {
    Expr::DataLiteral(Box::new(data_literal))
}

pub fn undefined() -> Expr {
    data_literal(DataLiteral::Undefined)
}

pub fn null() -> Expr {
    data_literal(DataLiteral::Null)
}

pub fn boolean(b: bool) -> Expr {
    data_literal(if b { DataLiteral::True } else { DataLiteral::False })
}

pub fn number(n: i64) -> Expr {
    data_literal(DataLiteral::Integer(n.to_string()))
}

pub fn decimal(n: &str) -> Expr {
    data_literal(DataLiteral::Decimal(n.to_string()))
}

pub fn string(s: &str) -> Expr {
    data_literal(DataLiteral::String(s.to_string()))
}

pub fn bigint(s: u64) -> Expr {
    data_literal(DataLiteral::Bigint(s.to_string()))
}

// Array

pub fn array(elements: Vec<Expr>) -> Expr {
    Expr::Array(Box::new(Array(elements)))
}

// Record

pub fn record(fields: Vec<PropDef>) -> Expr {
    Expr::Record(Box::new(Record(fields)))
}

pub fn keyvalue(key: &str, value: Expr) -> PropDef {
    PropDef::KeyValue(Box::new(Field::new_dynamic(SharedString::from_str(key))), value)
}
/* 
pub fn shorthand(key: &str, value: VariableCell) -> PropDef {
    PropDef::Shorthand(Box::new(Field::new_dynamic(SharedString::from_str(key))), value)
}
*/

// Patterns

pub fn rest<T: UnsafeInto<Pattern>>(pattern: T) -> Pattern {
    Pattern::Rest(Box::new(unsafe{pattern.unsafe_into()}))
}

// Functions

fn set_variable_pointers_for_pattern(pattern: &mut Pattern, decl_index: DeclarationIndex, property_access_chain: &mut Vec<PropertyAccess>) {
    match pattern {
        Pattern::Variable(cell) => {
            cell.ptr.set(decl_index.clone(), PropertyAccessChain::from_vec(property_access_chain.clone()));
        },
        Pattern::ArrayPattern(array) => {
            for (i, pattern) in array.0.iter_mut().enumerate() {
                property_access_chain.push(PropertyAccess::Element(i));
                set_variable_pointers_for_pattern(pattern, decl_index.clone(), property_access_chain);
                property_access_chain.pop();
            }
        },
        _ => unimplemented!()
    }
}

pub fn function_expr(name: Option<&str>, captures: Vec<(SharedString, DeclarationIndex)>, mut params: Vec<Pattern>, declarations: Vec<LocalDeclaration>, statements: Vec<Statement>) -> Expr {
    let mut decl_index = 0;
    let mut param_decls = params.iter_mut().map(|pattern| {
        set_variable_pointers_for_pattern(pattern, DeclarationIndex::Parameter(decl_index), &mut Vec::new());
        decl_index += 1;
        (*pattern).clone().into()
    }).collect::<Vec<ParameterDeclaration>>();

    decl_index = 0;
    let mut capture_decls = captures.into_iter().map(|(name, parent_index)| {   
        let capture_cell = VariableCell::initialized(name.clone(), parent_index, vec![]);

        CaptureDeclaration::Local {
            name: name,
            variable: capture_cell,

        }
    }).collect::<Vec<CaptureDeclaration>>();

    Expr::FunctionExpr(Box::new(Function{
        name: name.map(SharedString::from_str),
        captures: capture_decls,
        parameters: param_decls,
        declarations: declarations,
        statements: statements,
    }))
}

// BinaryExpr

fn binary_expr(op: BinaryOp, l: Expr, r: Expr) -> Expr {
    Expr::BinaryExpr(Box::new(BinaryExpr(op, l, r)))
}

pub fn add(l: Expr, r: Expr) -> Expr {
    binary_expr(BinaryOp::Add, l, r)
}

pub fn logical_and(l: Expr, r: Expr) -> Expr {
    binary_expr(BinaryOp::And, l, r)
}

pub fn logical_or(l: Expr, r: Expr) -> Expr {
    binary_expr(BinaryOp::Or, l, r)
}

pub fn mul(l: Expr, r: Expr) -> Expr {
    binary_expr(BinaryOp::Mul, l, r)
}

pub fn div(l: Expr, r: Expr) -> Expr {
    binary_expr(BinaryOp::Div, l, r)
}

pub fn modulo(l: Expr, r: Expr) -> Expr {
    binary_expr(BinaryOp::Mod, l, r)
}

pub fn sub(l: Expr, r: Expr) -> Expr {
    binary_expr(BinaryOp::Sub, l, r)
}

pub fn equal(l: Expr, r: Expr) -> Expr {
    binary_expr(BinaryOp::StrictEqual, l, r)
}

pub fn bitand(l: Expr, r: Expr) -> Expr {
    binary_expr(BinaryOp::BitAnd, l, r)
}


// Variable

pub fn variable(name: &str) -> Expr {
    Expr::Variable(Box::new(VariableCell::uninitialized(SharedString::from_str(name))))
}

pub fn variable_initialized(name: &str, declaration_index: DeclarationIndex) -> Expr {
    Expr::Variable(Box::new(VariableCell::initialized(SharedString::from_str(name), declaration_index, vec![])))
}

pub fn spread(expr: Expr) -> Expr {
    Expr::Spread(Box::new(expr))
}

pub fn property(expr: Expr, prop: &str) -> Expr {
    Expr::CallExpr(Box::new(CallExpr { expr, post_ops: vec![CallPostOp::Member(SharedString::from_str(prop))] }))
}

pub fn properties(expr: Expr, props: Vec<&str>) -> Expr {
    Expr::CallExpr(Box::new(CallExpr { expr, post_ops: props.into_iter().map(|prop| CallPostOp::Member(SharedString::from_str(prop))).collect() }))
}

pub fn shorthand(name: &str) -> PropDef {
    PropDef::Shorthand(Box::new(Field::new_dynamic(SharedString::from_str(name))), VariableCell::uninitialized(SharedString::from_str(name)))
}

pub fn paren(expr: Expr) -> Expr {
    Expr::ParenedExpr(Box::new(expr))
}

// Statements

pub fn return_statement(expr: Expr) -> Statement {
    Statement::Return(Box::new(expr))
}

pub fn const_declaration(name: &str, expr: Expr) -> LocalDeclaration {
    LocalDeclaration::Const{
        pattern: Pattern::Variable(Box::new(VariableCell::uninitialized(SharedString::from_str(name)))),
        value: Some(expr),
    }
}

pub fn const_statement(name: &str, decl: DeclarationIndex) -> Statement {
    Statement::LocalDeclaration(vec![decl])
}

pub fn let_declaration(name: &str, expr: Expr) -> LocalDeclaration {
    LocalDeclaration::Let{
        pattern: Pattern::Variable(Box::new(VariableCell::uninitialized(SharedString::from_str(name)))),
        value: Some(expr),
    }
}

pub fn let_statement(name: &str, decl: DeclarationIndex) -> Statement {
    Statement::LocalDeclaration(vec![decl])
}
/* 
pub fn capture(name: &str, declaration_index: DeclarationIndex) -> CaptureDeclaration {
    let name = SharedString::from_str(name);
    CaptureDeclaration::Local{
        name: name.clone(),
        variable: VariableCell::initialized(name.clone(), declaration_index, vec![])
    }
}
*/
pub fn block(statements: Vec<Statement>) -> Statement {
    Statement::Block(Block(statements))
}