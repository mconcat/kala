#![feature(allocator_api)]
#![feature(new_uninit)]
#![feature(const_trait_impl)]
#![feature(lazy_cell)]
#![feature(bigint_helper_methods)]
// #![feature(generic_const_exprs)]

pub mod array;
pub mod bigint;
pub mod number;
pub mod reference;
pub mod string;
pub mod slot;
pub mod operation;
pub mod function;

pub mod memory;