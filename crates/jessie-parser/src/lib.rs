#![recursion_limit = "256"]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(allocator_api)]
#![feature(test)]

//pub mod jessie_test;
pub mod parser;
pub mod lexer;
pub mod expression;
pub mod function;
pub mod statement;
pub mod pattern;
//pub mod module;
pub mod object;
pub mod operation;
pub mod scope;
// pub mod outline;
pub mod common;
mod map;
pub mod jessie_parser;

pub use jessie_parser::JessieParserState;
pub use lexer::{Lexer, Token};
pub use expression::expression;
//pub use module::*;

///////

//mod bench;