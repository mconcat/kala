use core::fmt::Debug;
use std::rc::Rc;
use jessie_ast::{Block, DeclarationKind, Declaration, Function, Scope};

use crate::jessie_scope::BlockScope;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError<C: Sized> {
    // A token is expected to be followed,
    // but a different token found.
    ExpectedToken(String, C),

    // Not a valid syntax in Jessie.
    InvalidExpression(String),

    // Valid syntax, but not implemented yet.
    Unimplemented(String),

    // Scoping error.
    ScopeError(String, String),
}


pub fn err_expected<R, C>(message: &'static str, actual: C) -> Result<R, ParserError<C>> {
    Err(ParserError::ExpectedToken(message.to_string(), actual))
}


pub fn err_invalid<R, C>(message: &'static str) -> Result<R, ParserError<C>> {
    Err(ParserError::InvalidExpression(message.to_string()))
}

pub fn err_unimplemented<R, C>(message: &'static str) -> Result<R, ParserError<C>> {
    Err(ParserError::Unimplemented(message.to_string()))
}

pub fn err_scope<R, C>(message: &'static str, var: String) -> Result<R, ParserError<C>> {
    Err(ParserError::ScopeError(message.to_string(), var))
}

/* 
pub trait ParserState: CombinatoryParser+Sized {
    pub fn get_parser(&mut self) -> &mut CombinatoryParserImpl; 

    const IS_PURE_JSON_RULE: bool;
    const IS_JESSIE_RULE: bool;
    const IS_TESSIE_RULE: bool;
/*
    pub fn ident(state: &mut Self) -> Result<String, String>; // TODO: not string maybe?

    type Expr: Debug+PartialEq+Clone+CommonExpr;
    pub fn CondExpr(condition: Self::Expr, consequent: Self::Expr, alternate: Self::Expr) -> Self::Expr;
    pub fn BinaryExpr(op: BinaryOp, left: Self::Expr, right: Self::Expr) -> Self::Expr;
    pub fn UnaryExpr(op: UnaryOp, operand: Self::Expr) -> Self::Expr;
    pub fn CallExpr(expr: Self::Expr, op: CallPostOp<Self>) -> Self::Expr;
    pub fn expr(state: &mut Self) -> Result<Self::Expr, String>;
    pub fn pure_expr(state: &mut Self) -> Result<Self::Expr, String>;
    pub fn primary_expr(state: &mut Self) -> Result<Self::Expr, String>;

    type Function: Debug+PartialEq+Clone;
    pub fn function_expr(state: &mut Self) -> Result<Self::Function, String>;
    pub fn function_decl(state: &mut Self) -> Result<Self::Function, String>;
    pub fn arrow_function(state: &mut Self) -> Result<Self::Function, String>;

    type Element: Debug+PartialEq+Clone;
    pub fn element(state: &mut Self) -> Result<Self::Element, String>;
    // pure element is always pure_expr

    type PropDef: Debug+PartialEq+Clone;
    pub fn prop_def(state: &mut Self) -> Result<Self::PropDef, String>;
    pub fn pure_prop_def(state: &mut Self) -> Result<Self::PropDef, String>;

    type PropName: Debug+PartialEq+Clone;
    pub fn prop_name(state: &mut Self) -> Result<Self::PropName, String>;

    type Variable: Debug+PartialEq+Clone;
    pub fn variable(state: &mut Self) -> Result<Self::Variable, String>;

    type TypeAnn: Debug+PartialEq+Clone;
    pub fn optional_type_ann(state: &mut Self) -> Option<Self::TypeAnn>;
    */
}
*/
/*
impl State {
    pub fn lookahead_1(&self) -> Option<char> {
        self.get_parser().lookahead_1()
    }

    pub fn lookahead_2(&self) -> Option<char> {
        self.get_parser().lookahead_2()
    }

    pub fn lookahead_3(&self) -> Option<char> {
        self.get_parser().lookahead_3()
    }

    pub fn lookahead_4(&self) -> Option<char> {
        self.get_parser().lookahead_4()
    }

    pub fn proceed(&mut self) {
        self.get_parser().proceed()
    }

    pub fn consume_1(&mut self, c: char) -> Result<(), String> {
        self.get_parser().consume_1(c)
    }

    pub fn consume(&mut self, s: &str) -> Result<(), String> {
        self.get_parser().consume(s)
    }

    pub fn consume_whitespace(&mut self) {
        self.get_parser().consume_whitespace()
    }

    pub fn lookahead_whitespace_nonident(&self) -> Result<(), String> {
        self.get_parser().lookahead_whitespace_nonident()
    }

    pub fn attempt<T>(&mut self, f: impl Fn(&mut Self) -> Result<T, String>) -> Result<T, String> {
        let state = self.get_parser();
        let pos = state.pos;
        match f(self) {
            Ok(r) => Ok(r),
            Err(err) => {
                state.pos = pos;
                Err(err)
            }
        }
    }

    pub fn prevent<T>(&mut self, f: impl Fn(&mut Self) -> Result<T, String>) -> Result<(), String> {
        let state = self.get_parser();
        let pos = state.pos;
        let result = match f(self) {
            Ok(_) => Err("Expected error, but got success".to_string()),
            Err(_) => Ok(()),
        };
        state.pos = pos;
        result
    }
}
*/
/* 
pub trait Node {
    fn parse(state: &mut ParserState) -> Result<Self, String> where Self: Sized;
}
*/


pub trait ArrayLike {
    type Element: PartialEq+Debug+Clone;

    fn get(&self, index: usize) -> Option<Self::Element>;
    fn len(&self) -> usize;
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserState<T: ArrayLike+Clone+Debug> {
    pub input: T,
    pub pos: usize,
    pub scope: BlockScope,
}

impl<T: ArrayLike+Clone+Debug> ParserState<T> {
    pub fn new(input: T) -> Self {
        Self {
            input,
            pos: 0,
            scope: BlockScope::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
//        println!("is_empty: {} >= {}", self.pos, self.input.len());
        self.pos >= self.input.len()
    }

    pub fn lookahead_1(&self) -> Option<T::Element> {
        self.input.get(self.pos)
    }

    pub fn lookahead_2(&self) -> Option<T::Element> {
        self.input.get(self.pos + 1)
    }

    pub fn lookahead_3(&self) -> Option<T::Element> {
        self.input.get(self.pos + 2)
    }

    pub fn lookahead_4(&self) -> Option<T::Element> {
        self.input.get(self.pos + 3)
    }

    pub fn lookahead_n(&self, n: usize) -> Option<T::Element> {
        if n == 0 {
            panic!("lookahead_n(0) is not allowed");
        }
        self.input.get(self.pos + n - 1)
    }

    pub fn proceed(&mut self) -> Option<T::Element> {
        let result = self.input.get(self.pos);
        if result.is_some() {
            self.pos += 1;
        }
        result
    }
    pub fn try_proceed(&mut self, c: T::Element) -> bool {
        if self.lookahead_1() == Some(c) {
            self.proceed();
            true
        } else {
            false
        }
    }

    pub fn consume_1(&mut self, c: T::Element) -> Result<(), ParserError<Option<T::Element>>> {
        if self.lookahead_1() == Some(c.clone()) {
            self.proceed();
            Ok(())
        } else {
            err_expected("consume_1", /*c.clone(),*/ self.lookahead_1())
        }
    }

    pub fn consume(&mut self, s: T) -> Result<(), String> {
        let pos = self.pos;

        for i in 0..s.len() {
            if self.lookahead_1() == s.get(i) {
                self.proceed();
            } else {
                self.pos = pos;
                return Err(format!("Expected {:?}, but got {:?}", s.get(i), self.lookahead_1()));
            }
        }

        Ok(())
    }

    pub fn enter_block_scope(&mut self) -> BlockScope {
        std::mem::replace(&mut self.scope, BlockScope::new())
    }

    /*
    Rc<Declaration> <- statement AST nodes
                    <- declarations(fixed)
                    <- bound_uses <- 
                    <- unbound_uses

     */

    pub fn exit_block_scope(&mut self, parent_scope: BlockScope) -> Scope {
        let scope = std::mem::replace(&mut self.scope, parent_scope); 
        let (declarations, unbound_uses) = scope.take();
        for (name, var) in unbound_uses {
            self.scope.assert_equivalence(&name, var);
        }

        Scope {
            declarations,
        }
    }

    pub fn exit_merge_block_scope(&mut self, parent_scope: BlockScope) {
        let scope = std::mem::replace(&mut self.scope, parent_scope);
        scope.merge_into(&mut self.scope);
    }

    /*
    # Define Javascript-style comments.
    _WS <- super._WS (EOL_COMMENT / MULTILINE_COMMENT)?   ${_ => SKIP};
    EOL_COMMENT <- "//" (~[\n\r] .)* _WS;
    MULTILINE_COMMENT <- "/*" (~"*/" .)* "* /" _WS;
    */





/* 
    pub fn left_paren(&mut self) -> Result<(), String> {
        self.consume_1('(')?;
        Ok(self.consume_whitespace())
    }

    pub pub fn right_paren(&mut self) -> Result<(), String> {
        self.consume_1(')')?;
        Ok(self.consume_whitespace())
    }

    pub pub fn left_brace(&mut self) -> Result<(), String> {
        self.consume_1('{')?;
        Ok(self.consume_whitespace())
    }

    pub pub fn right_brace(&mut self) -> Result<(), String> {
        self.consume_1('}')?;
        Ok(self.consume_whitespace())
    }

    pub pub fn left_bracket(&mut self) -> Result<(), String> {
        self.consume_1('[')?;
        Ok(self.consume_whitespace())
    }

    pub pub fn right_bracket(&mut self) -> Result<(), String> {
        self.consume_1(']')?;
        Ok(self.consume_whitespace())
    }
*/

/* 
    // attempt to parse, but if it fails, rewind the parser state
    // it does not backup or rollback any other state than pos
    pub fn attempt<R>(&mut self, f: impl Fn(&mut Self) -> Result<R, String>) -> Result<R, String> {
        let pos = self.pos;
        match f(self) {
            Ok(r) => Ok(r),
            Err(err) => {
                self.pos = pos;
                Err(err)
            }
        }
    }
    */

/*
    pub fn consume_keyword(&mut self, s: T) -> Result<(), String> {
        let pos = self.pos;
        self.consume(s)?;
        if self.lookahead_whitespace_nonident().is_err() {
            self.pos = pos;
            Err(format!("Expected whitespace, but got {}", self.lookahead_1().unwrap()))
        } else {
            Ok(())
        }
    }
*/

/* 
    pub fn try_consume_then<R>(&mut self, s: T, consequent: impl Fn(&mut Self) -> Result<R, String>, alternate: impl Fn(&mut Self) -> Result<R, String>) -> Result<R, String> {
        let pos = self.pos;
        match self.consume(s) {
            Ok(_) => consequent(self),
            Err(_) => {
                self.pos = pos;
                alternate(self)
            }
        }
    }
    */

    // backtrack the parser state to the last successful attempt
    // it backs up and rolls back including any other state than pos
    /* 
    pub fn backtrack<R>(&mut self, f: impl Fn(&mut Self) -> Result<R, String>) -> Result<R, String> {
        self.attempt(f) // TODO
    }

    pub fn prevent<R>(&mut self, f: impl Fn(&mut Self) -> Result<R, String>) -> Result<(), String> {
        let pos = self.pos;
        let result = match f(self) {
            Ok(_) => Err("Expected error, but got success".to_string()),
            Err(_) => Ok(()),
        };
        self.pos = pos;
        result
    }
*/
    pub fn proceed_then<R, C>(&mut self, r: R) -> Result<R, C> {
        self.proceed();
        Ok(r)
    }

    pub fn proceed_with<R>(&mut self, r: R) -> R {
        self.proceed();
        r
    }
    
/* 
    // used to parse variable names in context of declarations
    pub pub fn def_var(&mut self) -> Result<String, String> {

    }

    // used to parse variable names in context of reference
    pub pub fn use_var(&mut self) -> Result<String, String> {

    }

    // push one scope level
    pub pub fn push_scope(&mut self) {

    }

    // pop one scope level, returning the innermost scope constructed
    pub pub fn pop_scope(&mut self) -> Scope {

    }
    */
}

