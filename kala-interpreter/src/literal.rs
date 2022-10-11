
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NumberLiteral{
    SMI(i32),
//     Float(Decimal64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StringLiteral(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BooleanLiteral(pub bool); 

/* 
pub struct BigintLiteral {
    pub value: String,
}
*/

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Literal {
    Undefined,
    Number(NumberLiteral),
    Boolean(BooleanLiteral),
    String(StringLiteral),
   // Bigint(BigintLiteral),
}

impl From<kala_ast::ast::Literal> for Literal {
    fn from(literal: kala_ast::ast::Literal) -> Self {
        match literal {
            kala_ast::ast::Literal::Undefined => Literal::Undefined,
            kala_ast::ast::Literal::Boolean(b) => Literal::Boolean(BooleanLiteral(b.value)),
            kala_ast::ast::Literal::Number(n) => Literal::Number(NumberLiteral::SMI(n.value as i32)),
            kala_ast::ast::Literal::String(s) => Literal::String(StringLiteral(s.value)),
        }
    }
}