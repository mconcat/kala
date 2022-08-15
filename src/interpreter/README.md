# Kala Runtime

Runtime for Kala(Cosmos-compatible Jessie) and neccesary binding for Cosmwasm. 

- ./runtime: main implementation for crate::runtime
- ./cosmwasm: bindings for cosmwasm

## TODOs

- change variable bindings to Vec<> from HashMap<>
- implement lexical binding analysis
- implement variable category (stack, heap, captured, depending on the result of simple escape analysis)
- implement closure
- implement non-heap local object
- implement runtime borrow inference(so the interpreter can allocate RefCell<T> instead of Rc<RefCell<T>> for objects)

The main idea behind borrow inference is that we infer the category of the variable from the usage of the variable. Rust has few category of variables,
- "own"ed variables that has scope-bound lifetime(RAII, destruction at the end of scope)
- "borrow"ed variables that does not interfere with the memory allocation(no allocation/destruction, literally just referring the value)
- 

Jessie runs on single thread so the distinction between & and &mut is not a consideration here - all references are &mut, and there can be multiple &mut. The Rust way to implement this kind of shared mutability behaviour is to use `RefCell<T>`. When a caller wants to lend a mutable reference to a variable to the callee function, it can always pass the RefCell, as long as the variable does not escape above the lifetime.

A variable 'escapes' when it is held by other references. For example,

```javascript
function parent() {
    let x = {};
    let y = child(x);
    return y;
}

function child(obj) {
    let result = { obj };
    return result;
}
```

Value `x` and `y` are owned by function `parent`, based on RAII principle, they should be destructed at the end of the function. However this naturally causes problems:
- If `y` is freed, when the caller of `parent` tries to access on the returned value, it will cause a use-after-free.
- If `x` is freed, when the caller of `parent` tries to access on `y.obj`(if they could), it will cause a use-after-free.

In Rust, it is called "the lifetime of the result exceeds the lifetime of obj", and this is not representable through the lifetime parameter syntax and cannot be compiled. This is how Rust prevents escaping variables.

If we want to use escaping variables, we can use `Rc<RefCell<T>>` pattern. Using `Rc<T>` allocates the value on the heap, which follows reference counting garbage collection instead of RAII. The heap allocation will be made instead of owned / `RefCell`ed variable when the lifetime of the value exceedes the lifetime of the variable. Here are some examples:

```javascript
function f1() {
    let v = 3;
    return; // v never escapes local scope, interpreted into stack variable JSValue
}

function f2() {
    let v = {};
    return; // v never escapes local scope, interpreted into stack variable JSValue
}

function f3() {
    let v = {x: 0};
    for(let x = 0; x < 10; x++) {
        addOne(v); // v is passed to a function that takes reference, interpreted into stack variable RefCell<JSValue>
    }
    return; // v never escapes local scope, no Rc<>
}

function f4() {
    // this function borrows arr from the caller, and the result object has the same lifetime with the arr
    // the function is interpreted into something like Fn<'a>(RefCell<&'a JSValue>) -> RefCell<&'a JSValue>
    // but it cannot be expressed in a raw rust syntax
    let borrowsValue = (arr) => { return { arr } }; 

    let v = {};
    let result = borrowsValue(v); // array can be passed as reference to the stack variable because the result does not escape the parent scope
    return; // result does not escapes the local scope, thus not outlives the dependent value v, so it can be adsfsafhjs hffhj sajf
}
```