use crate::json_types::*;
use crate::parser::{self,};
use crate::lexer::{Token, repeated_elements};

type ParserState = parser::ParserState<Vec<Token>>;

/*
#[derive(Debug, PartialEq, Clone)]
pub struct JsonParser(ParserStateImpl);

ParserState for JsonParser {
    fn get_parser(&mut self) -> &mut crate::parser::CombinatoryParserImpl {
        &mut self.0
    }

    const IS_PURE_JSON_RULE: bool = true;
    const IS_JESSIE_RULE: bool = false;
    const IS_TESSIE_RULE: bool = false;

    type Expr = DataStructure<Self>;

    fn attempt<T>(&mut self, f: fn(&mut Self) -> Result<T, String>) -> Result<T, String> {
        
    }

    fn expr(state: &mut Self) -> Result<Self::Expr, String> {
        data_structure(state)
    }

    fn pure_expr(state: &mut Self) -> Result<Self::Expr, String> {
        pure_expr(state)
    }

    fn primary_expr(state: &mut Self) -> Result<Self::Expr, String> {
        data_structure(state)
    }

    type Function = Never;

    fn arrow_function(_: &mut Self) -> Result<Self::Function, String> {
        Err("arrow function not supported in JSON".to_string())
    }

    fn function_decl(_: &mut Self) -> Result<Self::Function, String> {
        Err("function declaration not supported in JSON".to_string())
    }

    fn function_expr(_: &mut Self) -> Result<Self::Function, String> {
        Err("function expression not supported in JSON".to_string())
    }
/*
    type Element = Self::Expr;

    fn element(state: &mut Self) -> Result<Self::Element, String> {
        data_structure(state)
    }  

    type PropDef = PropDef<Self>;

    fn prop_def(state: &mut Self) -> Result<Self::PropDef, String> {
        prop_def(state)
    } 

    fn pure_prop_def(state: &mut Self) -> Result<Self::PropDef, String> {
        pure_prop_def(state)
    }
*/
    type PropName = String;

    fn prop_name(state: &mut Self) -> Result<Self::PropName, String> {
        Self::ident(state)
    }

    type Variable = String;

    fn variable(state: &mut Self) -> Result<Self::Variable, String> {
        Self::ident(state)
    }

    fn ident(state: &mut Self) -> Result<String, String> {
        // copilot wrote, check and test later
        // seems like [a-zA-Z][a-zA-Z0-9]*
        let mut ident = String::new();

        match state.lookahead_1() {
            Some(x) if x.is_ascii_alphabetic() => {
                ident.push(x);
                state.proceed();
            }
            _ => return Err("Expected identifier".to_string()),
        }

        while let Some(x) = state.lookahead_1() {
            if x.is_ascii_alphanumeric() {
                ident.push(x);
                state.proceed();
            } else {
                break;
            }
        }
        Ok(ident)
    }

     type TypeAnn = Never;

     fn optional_type_ann(state: &mut Self) -> Option<Self::TypeAnn> {
        None
     }
}
*/
/*
#[derive(Debug, PartialEq, Clone)]
pub enum PureExpr<R: Rule> {
    DataLiteral(DataLiteral),
    Array(PureArray<R>),
    Record(PureRecord<R>),
    Hole,
}
*/

pub fn pure_expr(state: &mut ParserState) -> Result<DataStructure, String> {
    match state.lookahead_1() {
        Some('n') => {
            parse_null(state).map(DataStructure::DataLiteral)
        }
        Some('f') => {
            parse_false(state).map(DataStructure::DataLiteral)
        }
        Some('t') => {
            parse_true(state).map(DataStructure::DataLiteral)
        }
        Some('0'..='9') => {
            parse_number(state).map(DataStructure::DataLiteral)
        }
        Some('"') => {
            parse_string(state).map(DataStructure::DataLiteral)
        }
        Some('[') => {
            pure_array(state).map(DataStructure::Array)
        }
        Some('{') => {
            pure_record(state).map(DataStructure::Record)
        }
        Some(',') | Some(']') => Ok(DataStructure::DataLiteral(DataLiteral::Null)), // Not sure if this is correct
        Some(x) => Err(format!("Unexpected token: {}", x)),
        None => Err("Unexpected EOF".to_string()),
    }
}



pub fn data_structure(state: &mut ParserState) -> Result<DataStructure, String> {
    let result = match state.lookahead_1() {
        Some('n') | Some('f') | Some('t') | Some('0'..='9') | Some('"') => {
            data_literal(state).map(DataStructure::DataLiteral)
        }
        Some('[') => {
            array(state).map(DataStructure::Array)
        }
        Some('{') => {
            record(state).map(DataStructure::Record)
        }
        Some(x) => Err(format!("Unexpected token: {}", x)),
        None => Err("Unexpected EOF".to_string()),
    };

    state.consume_whitespace();

    result
}



pub fn parse_undefined(state: &mut ParserState) -> Result<(), String> {
    state.consume("undefined")?;
    state.lookahead_whitespace_nonident()?;
    state.consume_whitespace();
    Ok(())
}

pub fn parse_null(state: &mut ParserState) -> Result<DataLiteral, String> {
    state.consume("null")?;
    state.lookahead_whitespace_nonident()?;
    state.consume_whitespace();
    Ok(DataLiteral::Null)
}

pub fn parse_false(state: &mut ParserState) -> Result<DataLiteral, String> {
    state.consume("false")?;
    state.lookahead_whitespace_nonident()?;
    state.consume_whitespace();
    Ok(DataLiteral::False)
}

pub fn parse_true(state: &mut ParserState) -> Result<DataLiteral, String> {
    state.consume("true")?;
    state.lookahead_whitespace_nonident()?;
    state.consume_whitespace();
    Ok(DataLiteral::True)
}



pub fn data_literal(state: &mut ParserState) -> Result<DataLiteral, String> {
    match state.lookahead_1() {
        Some('n') => {
            parse_null(state)
        }
        Some('f') => {
            parse_false(state)
        }
        Some('t') => {
            parse_true(state)
        }
        Some('0'..='9') => {
            parse_number(state)
        }
        Some('"') => {
            parse_string(state)
        }
        Some(x) => Err(format!("Unexpected token: {}", x)),
        None => Err("Unexpected EOF".to_string()),
    }
}


/*
#[derive(Debug, PartialEq, Clone)]
pub struct PureArray<R: Rule> {
    pub elements: Vec<State::PureExpr>,
}
*/

fn pure_array(state: &mut ParserState) -> Result<Array, String> {
    repeated_elements(state, '[', ']', &pure_expr, false).map(|elements| Array(elements.into_iter().map(Element::Expr).collect()))
}

fn array(state: &mut ParserState) -> Result<Array, String> {
    repeated_elements(state, '[', ']', &data_structure, false).map(|elements| Array(elements.into_iter().map(Element::Expr).collect()))
}

/*
#[derive(Debug, PartialEq, Clone)]
pub struct PureRecord<R: Rule> {
    pub properties: Vec<PurePropDef<R>>,
}
*/

fn pure_record(state: &mut ParserState) -> Result<Record, String> {
    repeated_elements(state, '{', '}', &pure_prop_def, false).map(|properties| Record(properties))
}



fn record(state: &mut ParserState) -> Result<Record, String> {
    repeated_elements(state, '{', '}', &prop_def, false).map(|properties| Record(properties))
}
/* 
#[derive(Debug, PartialEq, Clone)]
pub struct PurePropDef<R: Rule> {
    pub key: State::PropName,
    pub value: PureExpr<R>,
}
*/

fn pure_prop_def(state: &mut ParserState) -> Result<PropDef, String> {
    let key = prop_name(state)?;
    state.consume_whitespace();
    state.consume(":")?;
    state.consume_whitespace();
    let value = pure_expr(state)?;
    Ok(PropDef::KeyValue(key, value))
}



fn prop_def(state: &mut ParserState) -> Result<PropDef, String> {
    let key = prop_name(state)?;
    state.consume_whitespace();
    state.consume(":")?;
    state.consume_whitespace();
    let value = data_structure(state)?;
    Ok(PropDef::KeyValue(key, value))
}

fn prop_name(state: &mut ParserState) -> Result<PropName, String> {
    match state.lookahead_1() {
        Some('"') => {
            parse_string(state).map(|x| PropName::String(x.to_string()))
        }
        Some('0'..='9') => {
            parse_number(state).map(|x| PropName::Number(x.to_string()))
        }
        Some(x) if x.is_ascii_alphabetic() => {
            parse_ident(state).map(|x| PropName::Ident(x))
        }
        Some(x) => Err(format!("Unexpected token: {}", x)),
        None => Err("Unexpected EOF".to_string()),
    }
}

fn parse_ident(state: &mut ParserState) -> Result<String, String> {
    let mut ident = String::new();
    while let Some(c) = state.lookahead_1() {
        if c.is_ascii_alphanumeric() || c == '_' {
            ident.push(c);
            state.proceed();
        } else {
            break;
        }
    }
    state.lookahead_whitespace_nonident()?;
    if ident == "__proto__" {
        Err("Ident cannot be __proto__".to_string())
    } else {
        Ok(ident)
    }
}