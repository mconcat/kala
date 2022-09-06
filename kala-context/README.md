# Context

This crate defines the runtime context trait that should be implemented by the execution runtime. The context represents the execution flow of a single interpretation, which could be a JIT interpreter, Rust-targeting transpiler, bytecode compiler, etc. The 