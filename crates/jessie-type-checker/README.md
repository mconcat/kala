# operational semantics sketch

https://rpeszek.github.io/posts/2022-01-03-ts-types-part3.html
https://users.soe.ucsc.edu/~abadi/Papers/FTS-submitted.pdf
https://uwspace.uwaterloo.ca/bitstream/handle/10012/13697/Arteca_Ellen.pdf
http://janvitek.org/pubs/ecoop15a.pdf

The purpose of the type checker is

1. To efficiently allocate memory for variables, with known type information
2. To efficiently allocate memory for local scope, with known type information(scope in js is just an object)

It is *not* for ensuring type safety of a program, as if we cannot determine a type of a variable, we can always fallback to `any`.

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