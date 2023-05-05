// "Justin"... sort of 


// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_Precedence
#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    // 3
    Or, // Left
    Coalesce, // Left

    // 4
    And, // Left

    // 5
    BitwiseOr, // Left

    // 6
    BitwiseXor, // Left

    // 7
    BitwiseAnd, // Left

    // 8 
    StrictEqual, // Left
    StrictNotEqual, // Left

    // 9 
    LessThan, // Left
    LessThanEqual, // Left
    GreaterThan, // Left
    GreaterThanEqual, // Left

    // 10
    BitwiseLeftShift, // Left
    BitwiseRightShift, // Left
    BitwiseUnsignedRightShift, // Left

    // 11
    Add, // Left
    Sub, // Left

    // 12
    Mul, // Left
    Div, // Left
    Mod, // Left

    // 13
    Pow, // Right
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOp { // preOp
    TypeOf,
    Pos,
    Neg,
    BitNot,
    Not,
}