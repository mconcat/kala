#![feature(once_cell)]

pub mod operation;
pub mod expression;
pub mod statement;
pub mod pattern;
pub mod function;
pub mod assignment;
pub mod object;
pub mod helper;
mod traits;

pub use operation::*;
pub use expression::*;
pub use statement::*;
pub use pattern::*;
pub use function::*;
pub use assignment::*;
pub use object::*;