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

JS is not a pure functional language, so Perceus cannot be directly applied. My plan is:
- Categorize all the common patterns of variables
- Find out how each of them can be statically analyzed and Rc-eliminated
- For the cases that cannot be analyzed, just fallback to `Rc<RefCell<Object>>`.

## Type inference

```rust
// https://rustdoc.swc.rs/swc_ecma_ast/enum.TsType.html
pub enum Type {
    TsAnyKeyword, // any
    TsUnknownKeyword, // unknown
    TsNumberKeyword, // number
    TsObjectKeyword, // object
    TsBooleanKeyword, // boolean
    TsBigIntKeyword, // bigint
    TsStringKeyword, // string
    // TsSymbolKeyword,
    // TsVoidKeyword,
    TsUndefinedKeyword, // undefined
    TsNullKeyword, // null
    TsNeverKeyword, // never
    // TsIntrinsicKeyword,

    TsFnType(TsFnType), // function (params)<type_params>: type_ann

    TypeRef(TsTypeRef), // type_name<type_params>
    // TypeQuery(TsTypeQuery), // typeof
    TypeLit(TsTypeLit), // { members }
    ArrayType(TsArrayType), // [ elem_type ]
    TupleType(TsTupleType), // (tuple_, element)
    OptionalType(TsOptionalType), // type_ann?
    RestType(TsRestType), // ...type_ann
    // UnionOrIntersectionType(TsUnionOrIntersectionType), // flatten
    UnionType(TsUnionType), // types | types 
    // IntersectionType(TsIntersectionType), // types & types // TODO
    // ConditionalType(TsConditionalType), 
    // InferType(TsInferType), // 
    ParenthesizedType(TsParenthesizedType), // (type_ann)
    // TypeOperator(TsTypeOperator),
    // KeyOfType() // TODO
    // ReadOnlyType(TsType) // readonly type_ann // TODO, not needed in inference yet
    // IndexedAccessType(TsIndexedAccessType), // obj_type[index_type] TODO
    // MappedType(TsMappedType),
    // LitType(TsLitType),
    NumberLitType(String),
    StringLitType(String),
    BigintLitType(Bigint),
    // TemplateLitType(String, ) // TODO
    // TypePredicate(TsTypePredicate), // TODO
    // ImportType(TsImportType), // TODO
}

pub enum TsTypeElement {
    // TsCallSignatureDecl(TsCallSignatureDecl), // function 
    // TsConstructSignatureDecl(TsConstructSignatureDecl),
    TsPropertySignature(TsPropertySignature), // property
    // TsGetterSignature(TsGetterSignature), // getter
    // TsSetterSignature(TsSetterSignature), // setter
    // TsMethodSignature(TsMethodSignature), // method
    // TsIndexSignature(TsIndexSignature),
}

pub enum Expr {
    DataLiteral(DataLiteral),
    Array(Array),
    Record(Record),
    ArrowFunc(Box<Function>),
    FunctionExpr(Box<Function>),
    Assignment(Box<Assignment>),
    CondExpr(Box<CondExpr>),
    BinaryExpr(Box<BinaryExpr>),
    UnaryExpr(Box<UnaryExpr>),
    CallExpr(Box<CallExpr>),
    // QuasiExpr()
    ParenedExpr(Box<Expr>),
    Variable(UseVariable),
}

let ctx: TypeContext<Variable, Type> = TypeContext::new();
let record: TypeRecord<String, Type> = TypeRecord::new();

let infer = (expr, expected) => match expr {
    DataLiteral(x) => match typeof x {
        False | True => TsBooleanKeyword.unify(expected),
        Number => TsNumberKeyword.unify(expected),
        Bigint => TsBigintKeyword.unify(expected),
        String => TsStringKeyword.unify(expected),
        Null => TsNullKeyword.unify(expected),
        Undefined => TsUndefinedKeyword.unify(expected),
    },
    Array(x) => match x.len() {
        0 => TsArrayType{expected.unwrap_or(TsAnyKeyword)},
        1 => TsArrayType{infer(x[0]).unify()}, // XXXXXXXXXX resume work from here
        _ => TsTupleType{x.map(infer)},
    },
    Record(x) => x.map(|propdef| {
        KeyValue(key, prop) => TsPropertySignature {
            key,
            readonly: false, // for now, add borrow inference later
            optional: false,
            init: None, // I didnt understand this one
            params: None, // For now
            // I think we can find all occurances to this property,
            // check all possibly types that can be assigned
            // and if it is less than five make it parameterized or something
            type_ann: Some(infer(prop)), // why option??
            type_params: None, // no generics for now
        }
        // MethodDef(MethodDef),// TODO
        Shorthand(var) => TsPropertySignature {
            var.name,
            readonly: false, // for now
            optional: false,
            init: None,
            params: None,
            type_ann: Some(infer(var)),
            type_params: None,
        }
        Spread(expr) => expr.unwrap_as_record().map(infer), // merge back to parent
    }),
    ArrowFunc(x) => TsFnType {
        params: x.parameters.borrow().unwrap_as_parameters().map(infer_pattern),
        type_params: None,
        type_ann: {
            let parent_scope = ctx.enter_scope();
            ctx.apply_scope(x.scope);
            let result = infer_statements(x.get_function_body());
            ctx.exit_scope(parent_scope);
            result
        },
    },
    FunctionExpr(x) => ctx.set(x.name, TsFnType {
        params: x.parameters.borrow().unwrap_as_parameters().map(infer_pattern),
        type_params: None,
        type_ann: {
            let parent_scope = ctx.enter_scope();
            ctx.set_function_name(x.name);
            ctx.apply_scope(x.scope);
            let result = infer_statements(x.get_function_body());
            ctx.exit_scope(parent_scope);
            result
        },
    }),
    Assignment(lvalue, op, expr) => match op {
        Assign => ctx.set(lvalue, infer(expr)),
        _ => ,
    },
    CondExpr(condition, consequent, alternative) => {
        // no assertion for condition as any value could be truthy in js
        let ctype = infer(consequent);
        let atype = assert(ctype, alternative);
        atype
    },
    BinaryExpr(op, x, y) => match op {
        // logical operator used not only for boolean values but also for short curcuit evals
        // and they does not need to be the same type
        Or => union(infer(x), infer(y)), 
        And => union(infer(x), infer(y)),
        Coalesce => union(assert_nullish(x), infer(y)),
        // bitwise operators require both types to be the same, and either number or bigint
        BitwiseOr|BitwiseXor|BitwiseAnd => assert(assert_numeric(x), y),
        // strict equality require both types to be the same
        StrictEqual|StrictNotEqual => assert(infer(x), y),
        // comparison operators require both types to be the, and either number or bigint
        LessThan|LessThanEqual|GreaterThan|GreaterThanEqual => assert(assert_numeric(x), y),
        // bitwise operators require both types to be the same, and either number or bigint
        BitwiseLeftShift|BitwiseRightShift => assert(assert_numeric(x), y),
        // ...except for >>>, which requires numbers
        BitwiseUnsignedRightShift => assert(assert_number(x), y),
        // add..... lets think about it later
        // sub mul div mod pow, all works for both number and bigint
        Sub|Mul|Div|Mod|Pow => assert(assert_numeric(x), y),
    },
    UnaryExpr(ops, x) => {
        let t = infer(x);
        ops.reverse().fold(|x, op| {
            match op {
                TypeOf => TsStringKeyword, // TODO: union type of type keywords
                Pos|Neg|BitNot => assert_numeric(x),
                Not => assert_boolean(x),
            }
        }, x)
    },
    CallExpr(x, op) => match op {
        Index(idx) => match infer(idx) {
            TsNumberKeyword => assert_arraylike(x),
            TsStringKeyword => assert_indexable(x),
            StringLitType(s) => assert_has_member(s, x),
            _ => assert_object(x),
        },
        Member(mem) => assert_has_member(mem, x),
        Call(args) => assert_callable(function_signature(args), x)
    },
    ParenedExpr(x) => infer(x),
    Variable(x) => ctx.get(x), 
}
```

## Lifetime inference

I am just writing down possible cases of variable usage patterns. I may miss some case. We can determine which category the variable belongs, and apply the memory management pattern accordingly.

In general, objects/variables can be classified into the following patterns:
- Primitive: the variable is pointing to a primitive, copyable value.
- Alias: the variable is used only once to give a shorthand name for certain expression.
- Referenced: the variable is used to read or write value from it, but no possible cases of accessing it outside of its declaration scope.
- Constructed: the variable is returned at the end of the function.
- Captured: the variable is captured by a closure(equivalent to the next case).
- Embedded: the variable is assigned as a field of a local variable.
- Escaped: the variable has escaped the scope and assigned to a variable from parent scopes(equivalent to the next case).
- Assigned: the variable is assigned to a field of a variable from parent scopes.

Lets break them down case by case

### Primitive

The variable is used as an alias for a primitive value. Primitive values are immutable thus assigning into a variable has equivalent behavior with simply inlining them. Note that variables are primitive, therefore can occure more then once.

```js
{
    let x = 3+4;
    let y = x*x;
    let s = "this is a string " + y;
}
```

Primitive variables are always copyable and owned.

### Alias

The variable is used one and only once after the declaration. The variable can be used for object field, operand, or function argument. This is similar to primitive values, but the variables need to occure once to retain the `owned` property. If the variable is an object or an array, all the internal references should be also owned in order to become owned(just like if a Rust struct cannot have reference to other values, unless the struct itself is lifetime parameterized.)

```js
{
    let field = { x: "this is the owned variable" }; // all the referenced field in the variable field is owned(following primitive variable rule).
    let object = field; // the variable field is no longer used after this point, we can treat the variable as `moved`.
    f(object); // the variable object is no longer used after this point, we can treat the function is taking the ownership.
}
```

Alias variables are
- copyable and owned if all the internal fields are also copyable and owned
- cloneable and owned else

### Referenced variable

The variable, and references to the variable does not escape, and all the occurances of the variable as argument can be passed as reference.

Variables could be referenced in Rust. A reference is a pointer to a stack(local) variable, which gets destructed at the end of the block. Since the lifetime of the variable is bound to the block where it is declared, if the variable is read and write inside such a block(and is not escaped), no heap allocation is needed and pointer to the stack could be safely used.

We do not need to distinguish immutable reference(`&`) and mutable reference(`&mut`) in Jessie. The main reason for disallowing shared mutable reference is to prevent concurrent access and race condition, which is not the case for single threaded JS execution environment. There are other reasons too but they are irrelevant for Jessie anyway. We will just use the symbol `&` for denoting stack pointer with lifetime bound, and it could be shared regardless of its mutability. The only constraint is that references should be dropped before the end of the variables lifetime.

Referenced/moved argument works same in Jessie too. For example, `console.log`, which prints debug data to the console, does not need to take the ownership and could be considered as a reference-taking function.

```js
console.log(obj: &any)
```

Reference variables are
- copyable if all the 

However, if we take an example of a constructor-like function 

```js
function SomeObject(x) {
    return {
        xfield: x,
    }
}
```

It becomes hard to determine whether the arguments are moved or borrowed. 

- If the argument used after 

If the arguments are moved:
- The arguments should not be used after calling SomeObject().
- The result object can act as a struct(inline fields), instead of an object(reference fields).

If the arguments are borrowed:
- The result object cannot exceed the lifetime of the arguments.
- The arguments can be used after calling SomeObject().

### Captured variable pattern

The variables can be captured by a closure. A closure can be conceptualized as an instantiation of non-capturing function pointer with type of `(scope: { localVariable1, localVariable2, ... }) => (arguments: []any) => Result`, parameterized by the scope. 

When a variable is captured by the closure, it could be thought as the variable being the field of the function object(more precisely, the function object is either 1. having the reference to the local variable or 2. the variable is moved to the function object and owned).

```js
() => {
    let x = {};
    let f = () => {
        // x is not used in any other places(=the static reference count of x within the block stays as 1(except for the declaration itself))
        // x could be considered to be moved and owned by f
        console.log(x); 
    }
    // f is not used hereafter; we can drop f and its owning variables at this point
    f()

    let y = {};
    let g = () => {
        console.log(y)
    }

    // y is occured twice, and we cannot apply occur-once alias pattern here.
    // we cannot move ownership of y to g because of this
    // however the argument console.log() is considered as borrowing, not moving
    // so the lowest intersection of the variable lifetime is &'a(where the 'a is this arrow block)
    // increase static reference count to 2
    console.log(y)

    // g is not used hereafter; at this point the reference count of g could be set to 0 and destructed
    // by destructing, the static refcount of its captured variables are also decremented,
    // statically inserting destruction code for `y` too.
    g()

    let z = {};
    let h = () => {
        // z is used nowhere else, could be moved to h
        console.log(z)
    };

    // h has now escaped the block scope.
    // since the captured variable z is moved to h, the h could be moved to the upper scope
    return h; 

    let w = {};
    let i = () => {
        // w is NOT moved to i
        console.log(i)
    }

    console.log(w) // because of this clause

    // i has now escaped the block scope, but holding w as reference, not ownership
    // thus we allocate w in heap, and make Rc<RefCell<Object>> of it to share.
    // I believe there are ways to optimize this... cannot think a better way
    return i
}
```

### Escaping variable pattern

https://medium.com/a-journey-with-go/go-introduction-to-the-escape-analysis-f7610174e890

The variables can be "escaped" if a variable is escaping its declared scope, in the following cases:
- the content of the variable can be assigned to a variable declared in upper scopes
- the variable can be returned from a function scope
- the variable can be assigned to a field of object with longer lifetime, creating reference from outside of its lifetime
- the variable is referenced / owned by an escaping variable

```js
{
    let upperscope = {};
    {
        let lowerscope = {};
        // value lowerscope escapes its lifetime, as variable upperscope now references the same lowerscope
        // if variable lowerscope does not escape 
        upperscope = lowerscope 
    }
}
```

### Argument pattern