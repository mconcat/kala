use std::borrow::BorrowMut;

use crate::lexer::{self, repeated_elements, Token, enclosed_element};
use crate::parser::{self, ArrayLike};
use crate::{jessie_types::*};
// use crate::json_parser::{self, parse_string, parse_null, parse_false, parse_number_or_bigint, parse_true, parse_undefined};
use crate::jessie_operation::*;
// use crate::jessie_scope::{Variable, VariableInternal};

impl ArrayLike for Vec<Token> {
    type Element = lexer::Token;

    fn get(&self, index: usize) -> Option<Self::Element> {
        self.get(index)
    }

    fn len(&self) -> usize {
        self.len()
    }
}

type ParserState = parser::ParserState<Vec<Token>>;

pub fn module_body(state: &mut ParserState) -> Result<ModuleBody, String> {
    let mut items = Vec::new();

    while let Some(_) = state.lookahead_1() {
        items.push(module_item(state)?);
    }

    Ok(ModuleBody(items))
}

pub fn module_item(state: &mut ParserState) -> Result<ModuleItem, String> {
    module_decl(state).map(ModuleItem::ModuleDecl) // TODO
}

pub fn module_decl(state: &mut ParserState) -> Result<ModuleDecl, String> {
    state.consume_1(Token::Const)?;
    repeated_elements(state, None, Token::Semicolon, &module_binding, false).map(ModuleDecl)
}

pub fn hardened_expr(state: &mut ParserState) -> Result<HardenedExpr, String> {
    Ok(HardenedExpr(expr(state)?))
}

pub fn module_binding(state: &mut ParserState) -> Result<ModuleBinding, String> {
    match state.lookahead_1() {
        Some(Token::LeftBrace) | Some(Token::LeftBracket) => {
            let pattern = binding_pattern(state)?;
            state.consume_1(Token::Equal)?;
            let expr = hardened_expr(state)?;
            Ok(ModuleBinding::PatternBinding(pattern, expr))
        },
        _ => {
            let ident = def_variable(state)?;
            state.consume_1(Token::Equal)?;
            let expr = hardened_expr(state)?; // TODO: check if right
            Ok(ModuleBinding::VariableBinding(ident, Some(expr)))
        }
    }
}

pub fn binding_pattern(state: &mut ParserState) -> Result<Pattern, String> {
    match state.lookahead_1() {
        Some(Token::LeftBracket) => repeated_elements(state, Some(Token::LeftBracket), Token::RightBracket, &param, false).map(|x| Pattern::ArrayPattern(x, optional_type_ann(state))),
        Some(Token::LeftBrace) => repeated_elements(state, Some(Token::LeftBrace), Token::RightBrace, &prop_param, false).map(|x| Pattern::RecordPattern(x, optional_type_ann(state))),
        _ => Err("Expected binding pattern".to_string()),
    }
}

// only parses original "pattern" rule from Jessica, not the entire variants of enum Pattern.
// consider changing the name to binding_or_ident_pattern or something...
pub fn pattern(state: &mut ParserState) -> Result<Pattern, String> {
    match state.lookahead_1() {
        Some(Token::LeftBracket) | Some(Token::LeftBrace) => binding_pattern(state),
        Some(Token::Comma) | Some(Token::RightBracket) => Ok(Pattern::Hole), // Not sure if its the right way...
        _ => // data_literal(state).map(|x| Pattern::DataLiteral(x)).or_else(|_| {
            ident(state).map(|x| Pattern::Variable(x, optional_type_ann(state)))
        //}),
    }
}

pub fn param(state: &mut ParserState) -> Result<Pattern, String> {
    if state.lookahead_1() == Some(Token::DotDotDot) {
        state.consume_1(Token::DotDotDot)?;
        return pattern(state).map(|x| Pattern::Rest(Box::new(x), optional_type_ann(state)))
    }

    let pat = pattern(state)?;
    if let Pattern::Variable(ref x, ref ann) = pat {
        if ann.is_some() {
            unimplemented!("Type annotations on parameters are not supported yet")
        }
        
        if state.try_proceed(Token::Equal) {
            let expr = expr(state)?;
            return Ok(Pattern::Optional(x.clone(), Box::new(expr), optional_type_ann(state)))
        }
    }

    Ok(pat)
}

pub fn prop_param(state: &mut ParserState) -> Result<PropParam, String> {
    if state.try_proceed(Token::DotDotDot) {
        return pattern(state).map(|x| PropParam::Rest(x))
    }

    let key = ident(state)?;

    match state.lookahead_1() {
        Some(Token::Colon) => {
            state.proceed();
            let pat = pattern(state)?;
            Ok(PropParam::KeyValue(key, pat))
        },
        Some(Token::Equal) => {
            state.proceed();
            let expr = expr(state)?;
            Ok(PropParam::Optional(key, expr))
        }
        _ => Ok(PropParam::Shorthand(key)),
    }
}

fn arrow_or_paren_expr(state: &mut ParserState) -> Result<Expr, String> {
    let args = repeated_elements(state, Some(Token::LeftParen), Token::RightParen, &param, true/*Check it*/)?;
    if state.try_proceed(Token::FatArrow) {
        match state.lookahead_1() {
            Some(Token::LeftBrace) => return Ok(Expr::ArrowFunc(Box::new(Function(None, args, None/*TODO */, block(state)?)))),
            _ => unimplemented!("Arrow functions without block body are not supported yet")
        }    
    } else {
        match args.as_slice() {
            &[p] => match p {
                Pattern::Variable(x, ann) => {
                    if ann.is_some() {
                        unimplemented!("Type annotations should not be in parenthesized expressions")
                    }
                    Ok(Expr::Variable(x))
                },
                Pattern::ArrayPattern(xs, ann) => {
                    unimplemented!("array literal in parenthesized expressions is not supported yet")
                },
                Pattern::RecordPattern(xs, ann) => {
                    unimplemented!("record literal in parenthesized expressions is not supported yet")
                },
                _ => Err("=> expected".to_string()),
            }
            _ => Err("Grouped expressions are not valid syntax in Jessie".to_string()),
        }
    }
}

fn function_or_cond_expr(state: &mut ParserState) -> Result<Expr, String> {
    if state.try_proceed(Token::Function) {
        let name = if let Some(Token::Identifier(name)) = state.lookahead_1() {
            state.proceed();
            Some(name)
        } else {
            None
        };

        let args = repeated_elements(state, Some(Token::LeftParen), Token::RightParen, &param, true)?;

        let ret_type = optional_type_ann(state);

        let body = block(state)?;

        Ok(Expr::FunctionExpr(Box::new(Function(name, args, ret_type, body))))
    } else {
        // jump to arithmetic expression parsing directly, as it is a variable(a primary expr)
        assignment_or_cond_expr(state)
    }
}
/*
fn typeof_or_cond_expr(state: &mut ParserState) -> Result<Expr, String> {
    // Same comment with above. also, because this is a prefix operator, 
    // it should recognize (, {, [, ` as valid trailing characters too.
    // for example, "typeof{}" is a valid expression.
    // TODO
    if state.try_proceed(Token::TypeOf) {
        let rest = unary_expr(state)?;
        Ok(Expr::UnaryExpr(Box::new(unary_expr_with_push(state, UnaryOp::TypeOf)?)))
    } else {
        assignment_or_cond_expr(state)
    }
}
*/
fn arrow_or_assignment_or_cond_expr(state: &mut ParserState) -> Result<Expr, String> {
    let lvalue = lvalue(state)?;
    let op = assign_op(state); // assign_op should not consume if fails(single token parser)
    if let Ok(op) = op {
        // at this point, we know it's an assignment expression, not cond expr.
        let rvalue = expr(state)?;
        return Ok(Expr::Assignment(Box::new(Assignment(lvalue, op, rvalue))))
    } 
    
    // If not an assignment, try single-parameter arrow function

    if let LValue::Variable(var) = lvalue {
        if state.try_proceed(Token::FatArrow) {
            match state.lookahead_1() {
                Some(Token::LeftBrace) => return Ok(Expr::ArrowFunc(Box::new(Function(None, vec![Pattern::Variable(var, None)], None/*TODO */, block(state)?)))),
                _ => unimplemented!("Arrow functions without block body are not supported yet")
            }    
        }
    }

    // If not an arrow function, it must be a cond expr

    cond_expr_with_initial(state, lvalue.into())

    /*
    if let Ok(result) = state.attempt(|state| {
        // TODO: this is inefficient but to optimize it we need to
        // change cond_expr into pratt parser.
        // using attempt instead of backtrack may cause memory leak or something,
        // because lvalue can be parsed twice, and registered in scope twice,
        // but the allocated lvalue is not freed, etc etc...
        // idk deal with it later
        let lvalue = lvalue(state)?;
        let op = assign_op(state);
        if let Ok(op) = op {
            // at this point, we know it's an assignment expression, not cond expr.
            let rvalue = expr(state)?;
            Ok(Expr::Assignment(Box::new(Assignment(lvalue, op, rvalue))))
        } else {

        }
        let rvalue = expr(state)?;
        Ok(Expr::Assignment(Box::new(Assignment(lvalue, op, rvalue))))
    }) {
        Ok(result)
    } else {
        cond_expr(state)
    }
    */
}

// assignExpr =
//   | ArrowFunc
//   | ParenedExpr
//   | FunctionExpr
//   | CondExpr // = CondExpr | BinaryExpr | UnaryExpr | CallExpr
//   | DataStructure
//   | QuasiExpr
//   | UseVariable
//   | LValue PostOp
//   | LValue AssignOp AssignExpr
pub fn expr(state: &mut ParserState) -> Result<Expr, String> {
    match state.lookahead_1() {
        // First parse parenthesized expressions, and then look for arrow symbol after. If it's an arrow, parse an arrow function. If not, parse a parenthesized expression, and take that as the primary expression, fallthrough to the last case.
        Some(Token::LeftParen) => arrow_or_paren_expr(state),

        // If the first identifier is "function", parse a function expression. 
        Some(Token::Function) => function_expr(state).map(|x| Expr::FunctionExpr(Box::new(x))),
    
        // If the first identifier is "typeof", parse a typeof expression. Else, treat it as a variable, and fallthrough to the last case.
        // Some('t') => typeof_or_cond_expr(state),

        // - Find if a unary operator is present
        Some(Token::Tilde) |
        Some(Token::Bang) |
        Some(Token::TypeOf) |
        Some(Token::Plus) |
        Some(Token::Minus) => cond_expr(state),

        // - If not, try to parse a lvalue. If if succeeds, try to parse an assignment operator. If it fails, if the lvalue is a variable, try parsing arrow, and if succeeds, jump to arrow function parsing. If not or failed, take the lvalue as a primary expression(=in case of index/field access, treat it as a callExpr with corresponding memberPostOp) and fallthrough to the next case.
        // - Try to parse a primary expression(or lvaue correspondence if fallthrough). If it fails, the expression is invalid. If it succeeds, lookahead the next token. If it is one of binary operators, or callPostOp, jump into the corresponding parser function. If not, return the primary expression.
        _ => arrow_or_assignment_or_cond_expr(state)
    }

// RD version for reference
/*
    state.attempt(arrow_func).map(|x| Expr::ArrowFunc(Box::new(x))).or_else(|_| 
        state.attempt(function_expr).map(|x| Expr::FunctionExpr(Box::new(x))).or_else(|_| 
            state.attempt(assignment).map(|x| Expr::Assignment(Box::new(x))).or_else(|_| 
                state.attempt(cond_expr).or_else(|_| 
                    primary_expr(state)
                )
            )
        )
    )
    */
}
/* 
pub fn assignment(state: &mut ParserState) -> Result<Assignment, String> {
    println!("assignment");
    let lvalue = lvalue(state)?;
    state.consume_whitespace();
    let op = assign_op(state)?;
    state.consume_whitespace();
    let expr = expr(state)?;
    Ok(Assignment(lvalue, op, expr))
}
*/

pub fn assign_op(state: &mut ParserState) -> Result<AssignOp, String> {
    match state.lookahead_1() {
        Some(Token::Equal) => state.proceed_then(AssignOp::Assign),
        Some(Token::PlusEqual) => state.proceed_then(AssignOp::AssignAdd),
        Some(Token::MinusEqual) => state.proceed_then(AssignOp::AssignSub),
        Some(Token::AsteriskEqual) => state.proceed_then(AssignOp::AssignMul),
        Some(Token::SlashEqual) => state.proceed_then(AssignOp::AssignDiv),
        Some(Token::PercentEqual) => state.proceed_then(AssignOp::AssignMod),
        _ => Err("sadf".to_string())
    }
    /*
    match state.lookahead_1() {
        Some('=') => {
            match state.lookahead_2() { // exclude binary operator cases that could start with =
                Some('=') | Some('>') => { // do we need to take care of arrow here?? idk, better safe than sorry
                    Err("sadf".to_string())
                }
                _ => {
                    state.consume("=")?;
                    Ok(AssignOp::Assign)
                }
            }
        },
        Some('+') => {
            state.consume("+=")?;
            Ok(AssignOp::AssignAdd)
        },
        Some('-') => {
            state.consume("-=")?;
            Ok(AssignOp::AssignSub)
        },
        Some('*') => {
            match state.lookahead_2() {
                Some('*') => {
                    state.consume("**=")?;
                    Ok(AssignOp::AssignExp)
                },
                Some('=') => {
                    state.consume("*=")?;
                    Ok(AssignOp::AssignMul)
                },
                _ => Err("sadf".to_string())
            }
        },
        Some('/') => {
            state.consume("/=")?;
            Ok(AssignOp::AssignDiv)
        },
        Some('%') => {
            state.consume("%=")?;
            Ok(AssignOp::AssignMod)
        },
        Some('<') => {
            state.consume("<<=")?;
            Ok(AssignOp::AssignLShift)
        },
        Some('>') => {
            match state.lookahead_3() {
                Some('=') => {
                    state.consume(">>=")?;
                    Ok(AssignOp::AssignRShift)
                },
                Some('>') => {
                    state.consume(">>>=")?;
                    Ok(AssignOp::AssignURShift)
                },
                _ => Err("Expected >> or >>>".to_string()),
            }
        },
        Some('&') => {
            state.consume("&=")?;
            Ok(AssignOp::AssignBitAnd)
        },
        Some('|') => {
            state.consume("|=")?;
            Ok(AssignOp::AssignBitOr)
        },
        Some('^') => {
            state.consume("^=")?;
            Ok(AssignOp::AssignBitXor)
        },
        _ => Err("Expected assignment operator".to_string()),
    }
    */
}

// lookaheads
// '(' => arrow func or paren expr
// '{' => purerecord
// '[' => purearray
// true, false, null, undefined, number, string => pureexpr
// other => variable
pub fn pure_expr(state: &mut ParserState) -> Result<Expr, String> {
    unimplemented!("pure_expr")

// RD parser for reference
    /*
    state.attempt(arrow_func).map(|x| Expr::ArrowFunc(Box::new(x))).or_else(|_| 
        state.attempt(json_parser::pure_expr).map(|x| x.into()).or_else(|_| 
            state.attempt(|state| {
                state.consume("(")?;
                state.consume_whitespace();
                let expr = pure_expr(state)?;
                state.consume_whitespace();
                state.consume(")")?;
                Ok(Expr::ParenedExpr(Box::new(expr)))
            }).or_else(|_| 
                ident(state).map(|x| Expr::Variable(x))
            )
        )
    )
    */
}

pub fn lvalue(state: &mut ParserState) -> Result<LValue, String> {
    let lval = primary_expr(state)?;

    match state.lookahead_1() {
        Some(Token::LeftBracket) => {
            state.proceed();
            let index = expr(state)?;
            state.consume_1(Token::RightBracket)?;
            Ok(LValue::Index(lval, index))
        },
        Some(Token::Dot) => {
            state.proceed();
            let name = ident(state)?;
            Ok(LValue::Member(lval, name))
        },
        _ => {
            match lval {
                Expr::Variable(name) => Ok(LValue::Variable(name)), // TODO
                _ => Err("Expected lvalue".to_string()),
            }
        }
    }
}

// lookaheads
// '{' => block_or_record
// const, let, function, if, while, try, continue, break, return, throw
// else go to expression

// statementItem
pub fn statement(state: &mut ParserState) -> Result<Statement, String> {
    // putting whitespace in consumes is a hack, need to fix later
    match state.lookahead_1() {
        Some(Token::LeftBrace) => block(state).map(Statement::Block), // TODO: implement statement level record literal?
        Some(Token::Const) => {
            state.proceed();
            declaration_body(state, DeclarationKind::Const).map(Statement::Declaration)
        },
        Some(Token::Let) => {
            state.proceed();
            declaration_body(state, DeclarationKind::Let).map(Statement::Declaration)
        },
        Some(Token::Function) => {
            state.proceed();
            function_decl_body(state).map(Statement::FunctionDeclaration)
        },
        Some(Token::If) => {
            state.proceed();
            if_statement(state).map(Statement::IfStatement)
        },
        Some(Token::While) => {
            state.proceed();
            while_statement(state).map(Statement::WhileStatement)
        },
        Some(Token::Try) => {
            unimplemented!("try statement")
            //state.proceed();
            //try_statement(state).map(Statement::TryStatement)
        },
        Some(Token::Throw) => {
            state.proceed();
            expr(state).map(Statement::Throw)
        },
        Some(Token::Continue) => {
            state.proceed();
            state.consume_1(Token::Semicolon)?;
            Ok(Statement::Continue)
        },
        Some(Token::Break) => {
            state.proceed();
            state.consume_1(Token::Semicolon)?;
            Ok(Statement::Break)
        },
        Some(Token::Return) => {
            state.proceed();
            if state.try_proceed(Token::Semicolon) {
                Ok(Statement::Return(None))
            } else {
                let e = expr(state)?;
                state.consume_1(Token::Semicolon)?;
                Ok(Statement::Return(Some(e)))
            }
        },
        _ => expr_statement(state)
    }
/*
    state.attempt(block).map(|x| Statement::Block(x)).or_else(|_| 
        state.attempt(if_statement).map(|x| Statement::IfStatement(x)).or_else(|_| 
            state.attempt(breakable_statement).map(|x| Statement::BreakableStatement(x)).or_else(|_| 
                state.attempt(terminator).map(|x| Statement::Terminator(x)).or_else(|_| 
                    //state.attempt(try_statement).map(|x| Statement::TryStatement(x)).or_else(|_| 
                        state.attempt(expr_statement).map(|x| Statement::ExprStatement(x))
                    // )
                )
            )
        )
    )
    */
}

pub fn expr_statement(state: &mut ParserState) -> Result<Statement, String> {
    // TODO: cantStartExprStatement
    let result = expr(state)?;
    state.consume_1(Token::Semicolon)?;
    Ok(Statement::ExprStatement(result))
}

pub fn block(state: &mut ParserState) -> Result<Block, String> {
    state.consume_1(Token::LeftBrace)?;

    let mut statements = Vec::new();
    while state.lookahead_1() != Some(Token::RightBrace) { // is it memory safe??
        statements.push(statement(state)?);
    }

    Ok(Block::new(statements))
}

pub fn if_statement(state: &mut ParserState) -> Result<IfStatement, String> {
    state.consume_1(Token::If)?;
    if_statement_body(state)
}

fn if_statement_body(state: &mut ParserState) -> Result<IfStatement, String> {
    // assume that "if" keyword is already consumed
    state.consume_1(Token::LeftParen)?;
    let condition = expr(state)?;
    state.consume_1(Token::RightParen)?;
    let consequent = block(state)?;

    let alternate = if state.try_proceed(Token::Else) {
        if state.try_proceed(Token::If) {
            Some(ElseArm::ElseIf(if_statement(state).map(Box::new)?))
        } else {
            Some(ElseArm::Body(block(state)?))
        }
    } else {
        None
    };

    Ok(IfStatement { condition, consequent, alternate })
}


pub fn while_statement(state: &mut ParserState) -> Result<WhileStatement, String> {
    state.consume_1(Token::While)?;
    while_statement_body(state)
}

pub fn while_statement_body(state: &mut ParserState) -> Result<WhileStatement, String> {
    // assume that "while" keyword is already consumed
    state.consume_1(Token::LeftParen)?;
    let condition = expr(state)?;
    state.consume_1(Token::RightParen)?;
    let body = block(state)?;

    Ok(WhileStatement { condition, body })
}
/*
pub fn terminator(state: &mut ParserState) -> Result<Terminator, String> {
    println!("terminator");
    match state.lookahead_1() {
        Some('c') => {
            state.consume("continue")?;
            state.consume_whitespace();
            state.consume(";")?;
            Ok(Terminator::Continue)
        },
        Some('b') => {
            state.consume("break")?;
            state.consume_whitespace();
            state.consume(";")?;
            Ok(Terminator::Break)
        },
        Some('r') => {
            println!("return start");
            state.consume("return")?;
            println!("return consumed");
            let arg = state.attempt(|state| {
                state.consume_whitespace();
                state.consume(";")?;
                Ok(None)
            }).or_else(|_| {
                // TODO: no newline for return argument
                state.lookahead_whitespace_nonident()?;
                state.consume_whitespace();
                let arg = expr(state)?;
                state.consume(";")?;
                Ok::<Option<Expr>, String>(Some(arg))
            })?;
            println!("return end");
            Ok(Terminator::Return(arg))
        },
        Some('t') => {
            state.consume("throw")?;
            state.lookahead_whitespace_nonident()?; // TODO: no newline
            state.consume_whitespace();
            let expr = expr(state)?;
            state.consume(";")?;
            Ok(Terminator::Throw(expr))
        },
        _ => Err("Expected terminator".to_string()),
    }
}
*/
pub fn declaration_body(state: &mut ParserState, kind: DeclarationKind) -> Result<Declaration, String> {
    // assume that "let" or "const" keyword is already consumed
    /*
    let kind = match state.lookahead_1() {
        Some('l') => state.consume("let").map(|_| DeclarationKind::Let),
        Some('c') => state.consume("const").map(|_| DeclarationKind::Const),
        _ => Err("Expected 'let' or 'const'".to_string()),
    }?;
    */

    let bindings = repeated_elements(state, None, Token::Semicolon, &binding, false)?;

    Ok(Declaration { kind, bindings })
}

pub fn binding(state: &mut ParserState) -> Result<Binding, String> {
    match state.lookahead_1() {
        Some(Token::LeftBrace) | Some(Token::LeftBracket) => {
            let pattern = binding_pattern(state)?;
            state.consume_1(Token::Equal)?;
            let expr = expr(state)?;
            Ok(Binding::PatternBinding(pattern, expr))
        },
        _ => {
            let name = ident(state)?;
            let expr = state.attempt(|state| {
                state.consume_1(Token::Equal)?;
                expr(state)
            }).ok();
            Ok(Binding::VariableBinding(name, expr))
        }
    }
}
// TODO: move to lexer
/* 
pub fn reserved_word(state: &mut ParserState) -> Result<ReservedWord, String> {
    match state.lookahead_1() {
        Some('n') => state.consume("null").map(|_| ReservedWord::Null),
        Some('f') => state.consume("false").map(|_| ReservedWord::False),
        Some('t') => state.consume("true").map(|_| ReservedWord::True),
        Some('a') => match state.lookahead_2() {
            Some('s') => state.consume("async").map(|_| ReservedWord::Async),
            Some('r') => state.consume("arguments").map(|_| ReservedWord::Arguments),
            _ => Err("Expected 'async' or 'arguments'".to_string()), 
        },
        Some('e') => state.consume("eval").map(|_| ReservedWord::Eval),
        Some('g') => state.consume("get").map(|_| ReservedWord::Get),
        Some('s') => state.consume("set").map(|_| ReservedWord::Set),
        _ => Err("Expected reserved word'".to_string()),
    }
}

pub fn reserved_keyword(state: &mut ParserState) -> Result<ReservedKeyword, String> {
    let reserved_word = match state.lookahead_1() {
        Some('c') => state.consume("class").map(|_| ReservedKeyword::Class),
        Some('d') => {
            match state.lookahead_2() {
                Some('e') => state.consume("delete").map(|_| ReservedKeyword::Delete),
                Some('o') => state.consume("do").map(|_| ReservedKeyword::Do),
                _ => Err("Expected 'delete' or 'do'".to_string()),
            }
        },
        Some('e') => state.consume("extends").map(|_| ReservedKeyword::Extends),
        Some('i') => {
            match state.lookahead_2() {
                Some('n') => {
                    match state.lookahead_3() {
                        Some('s') => state.consume("instanceof").map(|_| ReservedKeyword::InstanceOf),
                        Some(' ') => state.consume("in").map(|_| ReservedKeyword::In),
                        _ => Err("Expected 'instanceof' or 'in'".to_string()),
                    }
                },
                _ => Err("Expected 'instanceof' or 'in'".to_string()),
            }
        },
        Some('n') => state.consume("new").map(|_| ReservedKeyword::New),
        Some('s') => state.consume("super").map(|_| ReservedKeyword::Super),
        Some('t') => state.consume("this").map(|_| ReservedKeyword::This),
        Some('v') => state.consume("var").map(|_| ReservedKeyword::Var),
        Some('w') => state.consume("with").map(|_| ReservedKeyword::With),
        Some('y') => state.consume("yield").map(|_| ReservedKeyword::Yield),
        _ => Err("Expected a reserved keyword".to_string()),
    }?;

    state.lookahead_whitespace_nonident()?;

    Ok(reserved_word)
}

pub fn keyword(state: &mut ParserState) -> Result<Keyword, String> {
    let keyword = match state.lookahead_1() {
        Some('b') => state.consume("break").map(|_| Keyword::Break),
        Some('c') => {
            match state.lookahead_2() {
                Some('a') => match state.lookahead_3() {
                    Some('s') => state.consume("case").map(|_| Keyword::Case),
                    Some('t') => state.consume("catch").map(|_| Keyword::Catch),
                    _ => Err("Expected 'case' or 'catch'".to_string()),
                },
                Some('o') => match state.lookahead_4() {
                    Some('s') => state.consume("const").map(|_| Keyword::Const),
                    Some('t') => state.consume("continue").map(|_| Keyword::Continue),
                    _ => Err("Expected 'const' or 'continue'".to_string()),
                },
                _ => Err("Expected 'case', 'catch', 'const', or 'continue'".to_string()),
            }
        },
        Some('d') => {
            match state.lookahead_3() {
                Some('b') => state.consume("debugger").map(|_| Keyword::Debugger),
                Some('f') => state.consume("default").map(|_| Keyword::Default),
                _ => Err("Expected 'debugger' or 'default'".to_string()),
            }
        },
        Some('e') => {
            match state.lookahead_2() {
                Some('l') => state.consume("else").map(|_| Keyword::Else),
                Some('x') => state.consume("export").map(|_| Keyword::Export),
                _ => Err("Expected 'else' or 'export'".to_string()),
            }
        },
        Some('f') => {
            match state.lookahead_2() {
                Some('i') => state.consume("finally").map(|_| Keyword::Finally),
                Some('o') => state.consume("for").map(|_| Keyword::For),
                Some('u') => state.consume("function").map(|_| Keyword::Function),
                _ => Err("Expected 'finally', 'for', or 'function'".to_string()),
            }
        },
        Some('i') => {
            match state.lookahead_2() {
                Some('f') => state.consume("if").map(|_| Keyword::If),
                Some('m') => state.consume("import").map(|_| Keyword::Import),
                _ => Err("Expected 'if' or 'import'".to_string()),
            }
        },
        Some('r') => state.consume("return").map(|_| Keyword::Return),
        Some('s') => state.consume("switch").map(|_| Keyword::Switch),
        Some('t') => {
            match state.lookahead_2() {
                Some('h') => state.consume("throw").map(|_| Keyword::Throw),
                Some('r') => state.consume("try").map(|_| Keyword::Try),
                Some('y') => state.consume("typeof").map(|_| Keyword::TypeOf),
                _ => Err("Expected 'throw', 'try', or 'typeof'".to_string()),
            }
        },
        Some('v') => state.consume("void").map(|_| Keyword::Void),
        Some('w') => state.consume("while").map(|_| Keyword::While),
        _ => Err("Expected a keyword".to_string()),
    };

    if keyword.is_err() {
        return keyword
    }

    state.lookahead_whitespace_nonident()?;

    keyword
}

pub fn future_reserved_word(state: &mut ParserState) -> Result<FutureReservedWord, String> {
    match state.lookahead_1() {
        Some('a') => state.consume("await").map(|_| FutureReservedWord::Await),
        Some('e') => state.consume("enum").map(|_| FutureReservedWord::Enum),
        Some('i') => {
            match state.lookahead_2() {
                Some('m') => state.consume("implements").map(|_| FutureReservedWord::Implements),
                Some('n') => state.consume("interface").map(|_| FutureReservedWord::Interface),
                _ => Err("Expected 'implements' or 'interface'".to_string()),
            }
        },
        Some('p') => {
            match state.lookahead_2() {
                Some('a') => state.consume("package").map(|_| FutureReservedWord::Package),
                Some('r') => state.consume("protected").map(|_| FutureReservedWord::Protected),
                // TODO: private
                Some('u') => state.consume("public").map(|_| FutureReservedWord::Public), 
                _ => Err("Expected 'package' or 'protected'".to_string()),
            }
        },
        _ => Err("Expected a future reserved word".to_string()),
    }   
}
*/
fn array(state: &mut ParserState) -> Result<Array, String> {
    repeated_elements(state, Some(Token::LeftBracket), Token::RightBracket, &element, false).map(|elements| Array(elements))
}

fn record(state: &mut ParserState) -> Result<Record, String> {
    repeated_elements(state, Some(Token::LeftBrace), Token::RightBrace, &prop_def, false).map(|properties| Record(properties))
}

/* TODO: move to lexer
pub fn data_literal(state: &mut ParserState) -> Result<Expr, String> {
    let lit = json_parser::data_literal(state)?.into();
    Ok(Expr::DataLiteral(match lit {
        DataLiteral::Number(ref n) => {
            if state.lookahead_1() == Some('n') {
                state.proceed();
                DataLiteral::Bigint(n.clone())
            } else {
                lit
            }
        }
        _ => lit
    }))
}

pub fn data_structure(state: &mut ParserState) -> Result<Expr, String> {
    let result = match state.lookahead_1() {
        Some('n') | Some('f') | Some('t') | Some('0'..='9') | Some('"') => {
            data_literal(state)
        }
        Some('[') => {
            array(state).map(Expr::Array)
        }
        Some('{') => {
            record(state).map(Expr::Record)
        }
        Some('u') => {
            state.consume("undefined").map(|_| Expr::DataLiteral(DataLiteral::Undefined))
        }
        None => Err("Unexpected EOF".to_string()),
        _ => Err("Expected a data literal, array, record, or undefined".to_string()),
    };

    state.consume_whitespace();

    result
}
*/
/* 
enum IdentOrCandidate {
    Ident(String),
    Candidate(String),
}

// takes one possible candidate
fn ident_or_candidate(state: &mut ParserState, candidate: &str) -> Result<IdentOrCandidate, String> {
    let mut ident = String::new();
    let mut lookahead = state.lookahead_1();
    while lookahead.is_some() && lookahead.unwrap().is_ascii_alphanumeric() {
        ident.push(lookahead.unwrap());
        state.proceed();
        lookahead = state.lookahead_1();
    }

    if ident == candidate {
        Ok(IdentOrCandidate::Candidate(ident))
    } else {
        Ok(IdentOrCandidate::Ident(ident))
    }    
}
*/
// Lookaheads: 
// '(' => Paren
// '`' => Quasi // TODO
// '[' => DataStructure/Array
// '{' => DataStructure/Record
// '"' => DataStructure/DataLiteral/String
// 'n' ull => DataStructure/DataLiteral/Null
// 't' rue => DataStructure/DataLiteral/True
// 'f' alse => DataStructure/DataLiteral/False
// 'u' ndefined => DataStructure/DataLiteral/Undefined
// '0'..'9' => DataStructure/DataLiteral/Number
// 'f' unction => FunctionExpr
// ascii => use_variable
// did I miss anything? anything else????
pub fn primary_expr(state: &mut ParserState) -> Result<Expr, String> {
    match state.lookahead_1() {
        Some(Token::LeftParen) => {
            state.proceed();
            let e = expr(state)?;
            state.consume_1(Token::RightParen)?;
            Ok(Expr::ParenedExpr(Box::new(e)))
        },
        Some(Token::QuasiQuote) => unimplemented!("QuasiExpr not implemented"),
        Some(Token::LeftBracket) => array(state).map(Expr::Array),
        Some(Token::LeftBrace) => record(state).map(Expr::Record),
        Some(Token::String(s)) => state.proceed_then(Expr::DataLiteral(DataLiteral::String(s))),
        Some(Token::Number(n)) => state.proceed_then(Expr::DataLiteral(DataLiteral::Number(n))),
        Some(Token::Null) => state.proceed_then(Expr::DataLiteral(DataLiteral::Null)),
        Some(Token::True) => state.proceed_then(Expr::DataLiteral(DataLiteral::True)),
        Some(Token::False) => state.proceed_then(Expr::DataLiteral(DataLiteral::False)),
        Some(Token::Undefined) => state.proceed_then(Expr::DataLiteral(DataLiteral::Undefined)),
        Some(Token::Bigint(b)) => state.proceed_then(Expr::DataLiteral(DataLiteral::Bigint(b))),
        Some(Token::Function) => function_expr(state).map(|x| Expr::FunctionExpr(Box::new(x))),
        _ => use_variable(state).map(|x| Expr::Variable(x)),
    }
/*
    state.attempt(data_structure).map(|data| data.into()).or_else(|_| {
        match state.lookahead_1() {
            Some('(') => {
                state.consume("(")?;
                state.consume_whitespace();
                let expr = expr(state)?;
                state.consume(")")?;
                state.consume_whitespace();
                Ok(Expr::ParenedExpr(Box::new(expr)))
            },
            Some('`') => Err("QuasiExpr not implemented".to_string()),
            _ => Ok(ident(state).map(|x| {
                Expr::Variable(x)
            })?),
        }
    })*/
}

pub fn element(state: &mut ParserState) -> Result<Element, String> {
    if state.try_proceed(Token::DotDotDot) {
        let expr = expr(state)?;
        Ok(Element::Spread(expr))
    } else {
        let expr = expr(state)?;
        Ok(Element::Expr(expr))
    }
}

pub fn pure_prop_def(state: &mut ParserState) -> Result<PropDef, String> {
    if state.try_proceed(Token::DotDotDot) {
        let expr = expr(state)?;
        Ok(PropDef::Spread(expr))
    }
    // copilot generated these lines, didnt understand 
    /*  
    else if state.lookahead_1() == Some('[') {
        state.consume("[")?;
        state.consume_whitespace();
        let prop_name = prop_name(state)?;
        state.consume("]")?;
        state.consume_whitespace();
        if state.lookahead_1() == Some('(') {
            let method_def = method_def(state)?;
            Ok(PurePropDef::MethodDef(method_def))
        } else {
            Ok(PurePropDef::Parent(json::PurePropDef::Shorthand(prop_name)))
        }
    }
    */
    else {
        let prop_name = prop_name(state)?;
        if state.lookahead_1() == Some(Token::LeftParen) {
            unimplemented!()
            /* 
            let method_def = method_def(state)?;
            Ok(PurePropDef::MethodDef(method_def))
            */
        } else {
            Ok(PropDef::Shorthand(prop_name))
        }
    }
}

pub fn prop_def(state: &mut ParserState) -> Result<PropDef, String> {
    if state.try_proceed(Token::DotDotDot) {
        let expr = expr(state)?;
        Ok(PropDef::Spread(expr))
    }
    // copilot generated these lines, didnt understand 
    /*  
    else if state.lookahead_1() == Some('[') {
        state.consume("[")?;
        state.consume_whitespace();
        let prop_name = prop_name(state)?;
        state.consume("]")?;
        state.consume_whitespace();
        state.consume(":")?;
        state.consume_whitespace();
        let expr = expr(state)?;
        Ok(PropDef::KeyValue(prop_name, expr))
    } else if state.lookahead_1() == Some('*') {
        state.consume("*")?;
        state.consume_whitespace();
        let prop_name = prop_name(state)?;
        state.consume(":")?;
        state.consume_whitespace();
        let expr = expr(state)?;
        Ok(PropDef::KeyValue(prop_name, expr))
    */
    else if state.lookahead_1() == Some(Token::QuasiQuote) {
        Err("QuasiExpr not implemented".to_string())
    } else {
        let prop_name = prop_name(state)?;
        if state.try_proceed(Token::Colon) {
            let expr = expr(state)?;
            Ok(PropDef::KeyValue(prop_name, expr))
        } else {
            Ok(PropDef::Shorthand(prop_name))
        }
    }
}

pub fn cond_expr(state: &mut ParserState) -> Result<Expr, String> {
    println!("condexpr");
    let or_else_expr = or_else_expr(state)?;
    if state.try_proceed(Token::Question) {
        let expr1 = expr(state)?;
        state.consume_1(Token::Colon)?;
        let expr2 = expr(state)?;
        Ok(Expr::CondExpr(Box::new(CondExpr(or_else_expr, expr1, expr2))))
    } else {
        Ok(or_else_expr)
    }
}
pub fn or_else_expr(state: &mut ParserState) -> Result<Expr, String> {
    println!("orelseexpr");
    let mut result = and_then_expr(state)?;
    while state.try_proceed(Token::BarBar) {
        let and_then_expr2 = and_then_expr(state)?;
        result = Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Or, result, and_then_expr2)))
    }
    Ok(result)
}

pub fn and_then_expr(state: &mut ParserState) -> Result<Expr, String> {
    println!("andthenexpr");
    let mut result = eager_expr(state)?;
    while state.try_proceed(Token::AmpersandAmpersand) {
        let eager_expr2 = eager_expr(state)?;
        result = Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::And, result, eager_expr2)))
    }
    Ok(result)
}

pub fn eager_expr(state: &mut ParserState) -> Result<Expr, String> {
    let mut result = shift_expr(state)?;
    while let Some(la) = state.lookahead_1() {
        match la {
            Token::LAngle => result = state.proceed_then(Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::LessThan, result, shift_expr(state)?))))?,
            Token::RAngle => result = state.proceed_then(Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::GreaterThan, result, shift_expr(state)?))))?,
            Token::LAngleEqual => result = state.proceed_then(Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::LessThanEqual, result, shift_expr(state)?))))?,
            Token::RAngleEqual => result = state.proceed_then(Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::GreaterThanEqual, result, shift_expr(state)?))))?,
            _ => break,
        }
    }

    Ok(result)
}

pub fn shift_expr(state: &mut ParserState) -> Result<Expr, String> {
    let mut result = add_expr(state)?;
    while let Some(la) = state.lookahead_1() {
        match la {
            Token::LAngleLAngle => result = state.proceed_then(Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::BitwiseLeftShift, result, add_expr(state)?))))?,
            Token::RAngleRAngle => result = state.proceed_then(Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::BitwiseRightShift, result, add_expr(state)?))))?,
            Token::RAngleRAngleRAngle => result = state.proceed_then(Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::BitwiseUnsignedRightShift, result, add_expr(state)?))))?, 
            _ => break,
        }
    }

    Ok(result)
}

pub fn add_expr(state: &mut ParserState) -> Result<Expr, String> {
    let mut result = mult_expr(state)?;
    while let Some(la) = state.lookahead_1() {
        match la {
            Token::Plus => result = state.proceed_then(Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Add, result, mult_expr(state)?))))?,
            Token::Minus => result = state.proceed_then(Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Sub, result, mult_expr(state)?))))?,
            _ => break,
        }
    }

    Ok(result)
}

pub fn mult_expr(state: &mut ParserState) -> Result<Expr, String> {
    let mut result = pow_expr(state)?;
    while let Some(la) = state.lookahead_1() {
        match la {
            Token::Asterisk => result = state.proceed_then(Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Mul, result, pow_expr(state)?))))?,
            Token::Slash => result = state.proceed_then(Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Div, result, pow_expr(state)?))))?,
            Token::Percent => result = state.proceed_then(Expr::BinaryExpr(Box::new(BinaryExpr(BinaryOp::Mod, result, pow_expr(state)?))))?,
            _ => break,
        }
    }

    Ok(result)
}

pub fn pow_expr(state: &mut ParserState) -> Result<Expr, String> {
    // TODO. for now just route to unaryexpr.
    Ok(unary_expr(state)?)
}

pub fn unary_expr(state: &mut ParserState) -> Result<Expr, String> {
    // TODO, for now just route to callexpr.
    call_expr(state)
}

pub fn call_post_op(state: &mut ParserState) -> Result<CallPostOp, String> {
    match state.lookahead_1() {
        Some(Token::LeftBracket) | Some(Token::Dot) => Ok(CallPostOp::MemberPostOp(member_post_op(state)?)),
        Some(Token::LeftParen) => repeated_elements(state, Some(Token::LeftParen), Token::RightBrace, &arg, true).map(|args| CallPostOp::Call(args)),
        _ => Err(format!("Expected '[' or '.' or '('. Got: {:?}", state.lookahead_1()))
    }
}

pub fn call_expr(state: &mut ParserState) -> Result<Expr, String> { 
    println!("callexpr");
    let mut result = primary_expr(state)?;
    loop { // I don't like having an infinite loop here...
        match call_post_op(state) {
            Ok(op) => result = Expr::CallExpr(Box::new(CallExpr{ expr: result, post_op: op })),
            Err(_) => break,
        }
    }
    Ok(result)
}

pub fn member_post_op(state: &mut ParserState) -> Result<MemberPostOp, String> {
    match state.lookahead_1() {
        Some(Token::LeftBracket) => enclosed_element(state, Token::LeftBracket, Token::RightBracket, &expr).map(|e| MemberPostOp::Index(e)),
        Some(Token::Dot) => {
            state.proceed();
            let id = ident(state)?;
            Ok(MemberPostOp::Member(id))
        },
        _ => Err(format!("Expected '[' or '.'. Got: {:?}", state.lookahead_1()))
    }
}

pub fn arg(state: &mut ParserState) -> Result<Arg, String> {
    if state.try_proceed(Token::DotDotDot) {
        let e = expr(state)?;
        Ok(Arg::Spread(e))
    } else {
        let expr = expr(state)?;
        Ok(Arg::Expr(expr))
    }
}
/*
pub fn args(state: &mut ParserState) -> Result<Vec<Arg>, String> {
    state.consume("(")?;
    let mut args = vec![];
    if state.lookahead_1() == Some(')') {
        return Ok(args);
    }
    loop {
        args.push(arg(state)?);
        // TODO: check EOF
        if state.lookahead_1() == Some(')') {
            break;
        }
        state.consume(",")?;
        state.consume_whitespace();
    }
    Ok(args)
}
*/
pub fn function_decl(state: &mut ParserState) -> Result<Function, String> {
    state.consume_1(Token::Function)?;
    function_decl_body(state)
}

fn function_decl_body(state: &mut ParserState) -> Result<Function, String> {
    let name = ident(state)?;
    let params = repeated_elements(state, Some(Token::LeftParen), Token::RightParen, &param, false/*Check it*/)?;
    let body = block(state)?;
    Ok(Function(Some(name), params, optional_type_ann(state), body))
}

pub fn function_expr(state: &mut ParserState) -> Result<Function, String> {
    state.consume_1(Token::Function)?;
    function_expr_body(state)
}

fn function_expr_body(state: &mut ParserState) -> Result<Function, String> {
    println!("1");
    let name = state.attempt(|state| {
        def_variable(state)
    }).ok();
    println!("2");
    let params = repeated_elements(state, Some(Token::LeftParen), Token::RightParen, &param, false/*Check it*/)?;
    println!("3");
    let body = block(state)?;
    println!("4");
    Ok(Function(name, params, optional_type_ann(state), body))
}

pub fn arrow_func(state: &mut ParserState) -> Result<Function, String> {
    println!("arrow_func");
    let params = match state.lookahead_1() {
        Some(Token::LeftParen) => repeated_elements(state, Some(Token::LeftParen), Token::RightParen, &param, false/*Check it*/)?,
        _ => Err("unimplemented")?,//TODO // ident(state).map(|x| vec![ArrowParam(x)]),
    };

    // TODO: no_newline
    state.consume_1(Token::FatArrow)?;

    let body = match state.lookahead_1() {
        Some(Token::LeftBrace) => block(state)?,
        _ => Err("unimplemented")?, // TODO: expr(state),
    };
    
    Ok(Function(None, params, optional_type_ann(state), body))
}

fn optional_type_ann(state: &mut ParserState) -> Option<TypeAnn> {
    None // TODO
}

// use_variable, def_variable, def_function all parses ident(),
// but additionally do the hoisting/scoping work
// TODO: they should be rolled back in case of attempt(). 
/* 
fn use_variable(state: &mut ParserState) -> Result<String, String> {
    // check the following:
    // 1. the variable is defined in the current scope => use it.
    // 2. the variable is defined in the parent scope. => push it to preaccess with the parent as candidate
    // 3. the variable is not defined => push it to preaccess
    let name = ident(state)?;

    let scope = state.current_scope();

    if let Some(var) = scope.get_local_binding(&name) {
        return Ok(var)
    }

    if let Some(var) = scope.get_parent_binding(&name) {
        let local_var = scope.make_possibly_preaccess_variable(&name, var);
        return Ok(local_var)
    }
    
    Ok(scope.make_preaccess_variable(&name))
}

fn def_variable(state: &mut ParserState) -> Result<String, String> {
    // check the following:
    // 1. the variable is already defined in the current scope => error
    // 2. the variable is preaccessed in the current scope => hoist(but unitialized)
    // 3. else, bind it.

    let name = ident(state)?;

    let scope = state.current_scope();

    if let Some(mut var) = scope.get_local_binding(&name) {
        if var.is_declared() {
            return Err(format!("Variable {} is already defined in the current scope", name))
        }

        *var.borrow_mut() = scope.define_variable(&name);
        return Ok(var)
    }

    Ok(scope.define_variable(&name))
}
*/
/* 
fn def_function(state: &mut ParserState) -> Result<String, String> {
    // check the following:
    // 1. the function is already defined in the current scope => error
    // 2. the variable is preaccessed in the current scope => bind it(value hoisting),
    // 3. else, just bind it

    let name = ident(state)?;

    let scope = state.current_scope();

    if let Some(mut var) = scope.get_local_binding(&name) {
        if var.is_declared() {
            return Err(format!("Function {} is already defined in the current scope", name))
        }

        // bind it to the existing Rc pointer. this will bind to all the existing
        // reference from the current scope.
        // TODO: this still not does value hoisting(only reference hositing)
        // make it work.... somehow idk
        *var.borrow_mut() = scope.define_variable(&name);
        return Ok(var)
    }

    Ok(scope.define_variable(&name))
}
*/
fn prop_name(state: &mut ParserState) -> Result<PropName, String> {
    // TODO: datastructure propname
    ident(state).map(|name| PropName::Ident(name ))
}

fn ident(state: &mut ParserState) -> Result<String, String> {
    if let Some(Token::Identifier(id)) = state.lookahead_1() {
        state.proceed();
        Ok(id)
    } else {
        Err("Expected identifier".to_string())
    }
}

/*
// ident prevents reserved_keyword, reserved_word, future_reserved_word
// TODO: move to lexer
fn ident(state: &mut ParserState) -> Result<String, String> {
    // could be optimized
    state.prevent(reserved_keyword)?;
    state.prevent(reserved_word)?;
    state.prevent(future_reserved_word)?;

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


*/