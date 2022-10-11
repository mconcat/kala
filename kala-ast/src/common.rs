use core::panic;

use swc_ecma_ast as ast;

#[derive(Debug, Clone)]
pub enum DeclarationKind {
    Let,
    Const,
}

impl DeclarationKind {
    pub fn is_mutable(&self) -> bool {
        match self {
            DeclarationKind::Let => true,
            DeclarationKind::Const => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
}


impl From<ast::Ident> for Identifier {
    fn from(ident: ast::Ident) -> Self {
        Identifier {
            name: ident.sym.to_string(),
        }
    }
}

////////////////////////////////////////////////////////////////////////
/// Literals

#[derive(Debug, Clone)]
pub struct NumberLiteral{
    pub value: f64, // TODO: use decimal64
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct BooleanLiteral {
    pub value: bool,
}

#[derive(Debug, Clone)]
pub struct BigintLiteral {
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Undefined,
    Number(NumberLiteral),
    Boolean(BooleanLiteral),
    String(StringLiteral),
}

const MAX_SAFE_INTEGER: f64 = 9007199254740991.0;
const MIN_SAFE_INTEGER: f64 = -9007199254740991.0;

impl From<f64> for NumberLiteral {
    fn from(num: f64) -> Self {
        NumberLiteral{value: num}
    }
}

impl From<ast::Lit> for Literal {
    fn from(lit: ast::Lit) -> Self {
        match lit {
            ast::Lit::Str(str) => Literal::String(StringLiteral{value: str.value.to_string()}),
            ast::Lit::Num(num) => Literal::Number(num.value.into()), // TODO: add bound checks
            ast::Lit::Bool(bool) => Literal::Boolean(BooleanLiteral{value: bool.value}),
            ast::Lit::Null(_) => panic!(),
            // ast::Lit::BigInt(bigint) => Literal::Bigint(bigint.value.to_string()),
            _ => unimplemented!(),
        }
    }
}


