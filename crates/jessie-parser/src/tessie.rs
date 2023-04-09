use crate::{jessie_types};

/* 
#[derive(Debug, PartialEq, Clone)]
pub struct TessieRule();

impl Rule for TessieRule {
    const IS_PURE_JSON_RULE: bool = false;
    const IS_JESSIE_RULE: bool = false;
    const IS_TESSIE_RULE: bool = true;
}

#[derive(Debug, Clone, PartialEq)]
pub enum TsType {
    TsKeywordType(TsKeywordType),
    TsTypeLit(TsTypeLit),
    TsArrayType(TsArrayType),
    TsTupleType(TsTupleType),
    TsOptionalType(TsOptionalType),
    TsUnionType(TsUnionType),
    TsLitType(TsLitType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TsKeywordType {
    TsAnyKeyword,
    TsNumberKeyword,
    TsObjectKeyword,
    TsBooleanKeyword,
    TsBigIntKeyword,
    TsStringKeyword,
    TsUndefinedKeyword,
    TsNullKeyword,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TsTypeLit {
    pub members: Vec<TsTypeElement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TsTypeElement {
    TsCallSignatureDecl(TsCallSignatureDecl),
    TsPropertySignature(TsPropertySignature),
    TsGetterSignature(TsGetterSignature),
    TsSetterSignature(TsSetterSignature),
    TsMethodSignature(TsMethodSignature),
    TsIndexSignature(TsIndexSignature),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TsArrayType(pub Box<TsType>);

#[derive(Debug, Clone, PartialEq)]
pub struct TsTupleType(pub Vec<TsType>);

#[derive(Debug, Clone, PartialEq)]
pub struct TsOptionalType(pub Box<TsType>);

#[derive(Debug, Clone, PartialEq)]
pub struct TsUnionType(pub Vec<TsType>);

#[derive(Debug, Clone, PartialEq)]
pub enum TsLitType {
    Number(String),
    Str(String),
    Bool(bool),
    BigInt(String),
}

// Typed patters are used for variable declaration and function parameter declarations.
pub struct Pattern<R: Rule>(pub jessie_types::Pattern<R>, pub Option<TsType>);

// Function is used for function declaration, function expressions, and arrow functions.
pub struct Function<R: Rule>(pub Option<String>, pub Vec<Pattern/*Param*/<R>>, pub Option<TsType>, pub jessie_types::Block/*TODO: OrExpr*/<R>);
*/