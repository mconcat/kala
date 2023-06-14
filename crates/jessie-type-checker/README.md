# operational semantics sketch

https://rpeszek.github.io/posts/2022-01-03-ts-types-part3.html
https://users.soe.ucsc.edu/~abadi/Papers/FTS-submitted.pdf
https://uwspace.uwaterloo.ca/bitstream/handle/10012/13697/Arteca_Ellen.pdf
http://janvitek.org/pubs/ecoop15a.pdf

The purpose of the type checker is

1. To efficiently allocate memory for variables, with known type information
2. To efficiently allocate memory for local scope, with known type information(scope in js is just an object)

It is *not* for ensuring type safety of a program, as if we cannot determine a type of a variable, we can always fallback to `any`.

Also, this repo will perform the borrow inference. Typical JS engines employs a GC to make dynamic memory allocations, but we are trying to make a RC-based memory management system. RC based approach is considered as deprecated compared to GC, as they have constant overhead for each reference types, hence inferior performance. But recent researches like Perceus detects and eliminates unused reference counting.

## Type checker

1. Variable Assignment: Assign variables to all the top level functions(their parameters and return type).
1. Inference-from-definition: Make bidirectional-local-row type inference for each top level functions. This will ensure that the inference time depends only on the code size.
1. Inference-from-usage: After the type informations got unified, we can use them as a ground facts, to infer more concrete type from the usages

I'm pretty sure there will be a technical term for two different types of inference but don't know exactly. I will use the term "lowerbound inference" for the local inference for each functions, and "upperbound inference" for the global inference by their actual usages.

## 0.1 PoC

For Proof of Concept

- Support static property access, without prototypes
- No generic type inference, objects extending the parameter type should be wrapped with a type adapter. ref: https://www.tapirgames.com/blog/golang-interface-implementation https://brson.github.io/rust-anthology/1/all-about-trait-objects.html 
- Generic types(such as type of Array.map) takes generic type as its implicit parameter. ref: https://gitlab.haskell.org/ghc/ghc/-/wikis/commentary/rts/haskell-execution/function-calls. Generic types cannot be constrainted(`extends`) and can only be used for container-like types or generic functions
- Type inference for function calls applied only for direct call, not indirect call. Indirect calls take implicit type parameter at the runtime.

With these we can still have a O(1) time access for most of the properties, with few indirections via type parameters or interface adapter.