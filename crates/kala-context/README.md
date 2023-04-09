# Context

This crate defines the runtime context trait that should be implemented by the execution runtimes.

Context module has two different types of context, one for interpretation and one for transpilation. `EvaluationContext` is an AST traversal trait with 1-to-1 correspondence to AST node. `GenerationContext` constructs foreign code from the input code with methods exposing primitive components of target language. 

`JSValue` and `JSVariable` are both 