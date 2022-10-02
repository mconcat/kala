use swc_ecma_ast as ast;

pub enum DeclarationKind {
    Let,
    Const,
}

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

pub struct NumberLiteral{
    pub value: f64, // TODO: use decimal64
}

pub struct StringLiteral {
    pub value: String,
}

pub struct BooleanLiteral {
    pub value: bool,
}

pub struct BigintLiteral {
    pub value: String,
}

pub enum Literal {
    Undefined,
    Null,
    Number(NumberLiteral),
    Boolean(BooleanLiteral),
    String(StringLiteral),
    Bigint(BigintLiteral),
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
            ast::Lit::Null(_) => Literal::Null,
            // ast::Lit::BigInt(bigint) => Literal::Bigint(bigint.value.to_string()),
            _ => unimplemented!(),
        }
    }
}
