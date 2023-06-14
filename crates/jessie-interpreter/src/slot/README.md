# Slot

The slot is the data type that is being stored inside the stack frame, covering primitive, reference, vm internal data types. The name and the design is influenced by the [XS slot type](https://github.com/Moddable-OpenSource/moddable/blob/public/documentation/xs/XS%20in%20C.md).

Here are the assumptions we will have while designing the interpreter:
- The intepreter will be run on either a 64-bit real world machine or a WASM environment. Embedded machines like 32-bit based or blockchain-specific VMs including EVM should be supported in the future but have lower priority.
- The memory usage will be limited and 32-bit pointer will be sufficient to cover the whole program. 
- The memory is 64-bit word aligned; no matter the machine is 32-bit based or 64-bit based.
- The bytecode might be malicious. If the compilation has been done by an external party, there should be some isolation mechanism while executing the bytecode(both machine-program and program-program).
- Concurrency/parallelism is not a responsibility of the interpreter. An event-loop based single thread concurrency mechanism could be provided by the engine.
- We may want to design the bytecode (at least partially) compatible with the XS bytecode set, as they support the whole embedded programming field.
- Determinism is an absolute requirement, so we cannot use any concurrent GC technique(CMS can cause nondeterministic out of memory)
- GC cannot be used in general because of its globally synchronous 'stop-the-world' behavior.
- No prototype, no classes(Jessie design). They could be written in a spread operator form to copy and paste fields from a template object, and the parser should optimize not to duplicate all the functions in such cases.
- Reference types could be inlined in the stack when they have a known size(such as tuples or index-less objects). Heap allocation or stack allocation should not effect on the semantics.
- We conceptually model a block scope as a known object. Here is an example
```js
{ // #1
    let a = 3;
    const b = 4;
    let o = { prop: undefined };
    { // #2
        const a = 5;
        function f() { return a; }
        o.prop = f;
    }
    a = o.prop();
}
```

becomes 

```ts
interface Scope#1 {
    let a: number;
    const b: number;
    let o: { let prop: () => number };
}

interface Scope#2 {
    const a: number;
    function f(): number;
    let o: { let prop: () => number }; // reference to the parent stack variable.
}
```

Note that `let` and `const` are declaration modifiers, but we use them internally also for the mutable/immutable binding for interface properties(like readonly/non-readonly). This could be inferred for normal object properties too(by the typechecker), and could be used for further optimization. `function` works same for `const` in terms of mutability, so the compiler could decide to reuse single function object instead of duplicate them.

The objects could be inlined if they have a known size, and in case of block scopes, the variables(properties) are all known, so this leads us to scope variables being stack allocated.

