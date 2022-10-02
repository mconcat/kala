
pub enum NumberLiteral{
    SMI(i32),
    Float(Decimal64),
}

pub struct StringLiteral(String);

pub struct BooleanLiteral(bool); 

/* 
pub struct BigintLiteral {
    pub value: String,
}
*/

pub enum Literal {
    Undefined,
    Null,
    Number(NumberLiteral),
    Boolean(BooleanLiteral),
    String(StringLiteral),
   // Bigint(BigintLiteral),
}