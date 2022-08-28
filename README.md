# kala.js

Kala is an interchain scripting layer on the Cosmos ecosystem. Kala provides Jessie-compatible scripting with interpreter/transpiler running on Cosmwasm and Cosmos-SDK.

## Babel

`babel` directory contains offchain integration with Bable transpiler suite to blacklist non-Jessie syntax from the input script, and encode it with a `probe` serialization format.

TODO: switch from babel to swc parser

## runtime

`src/runtime` directory contains the interface for transpiler backend. With the probe encoded Jessie AST, `runtime/lexical.rs` do the hoisting / binding / lexical analysis on it, which is then transpiled into provided runtime context. The `Context` trait exposes a common requirement for running a Jessie script, which could be a simple runtime interpreter or Rust-targeting transpiler. 