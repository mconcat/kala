const stmt: Stdfsa = ExprStatement(CallExpr(CallExpr {
    expr: ParenedExpr(Function(Function {
        name: Arrow,
        parameters: [],
        captures: [],
        locals: [
            Variable {
                name: SharedString("x"),
                index: Cell { value: Local(0) },
                block_index: 0,
            },
            Variable {
                name: SharedString("f"),
                index: Cell { value: Local(1) },
                block_index: 0,
            },
        ],
        functions: [(
            Variable {
                name: SharedString("f"),
                index: Cell { value: Local(1) },
                block_index: 0,
            },
            Function {
                name: Named(SharedString("f")),
                parameters: [],
                captures: [Variable {
                    name: SharedString("x"),
                    index: Cell { value: Local(0) },
                    block_index: 0,
                }],
                locals: [],
                functions: [],
                statements: Block {
                    statements: [Return(Variable(Variable {
                        name: SharedString("x"),
                        index: Cell { value: Capture(0) },
                        block_index: 0,
                    }))],
                },
            },
        )],
        statements: Block {
            statements: [
                LocalDeclaration(Const([VariableDeclaration {
                    pattern: Variable(Variable {
                        name: SharedString("x"),
                        index: Cell { value: Local(0) },
                        block_index: 0,
                    }),
                    value: Some(DataLiteral(Integer(3))),
                }])),
                LocalDeclaration(Function(Function {
                    name: Named(SharedString("f")),
                    parameters: [],
                    captures: [Variable {
                        name: SharedString("x"),
                        index: Cell { value: Local(0) },
                        block_index: 0,
                    }],
                    locals: [],
                    functions: [],
                    statements: Block {
                        statements: [Return(Variable(Variable {
                            name: SharedString("x"),
                            index: Cell { value: Capture(0) },
                            block_index: 0,
                        }))],
                    },
                })),
                Return(CallExpr(CallExpr {
                    expr: Variable(Variable {
                        name: SharedString("f"),
                        index: Cell { value: Local(1) },
                        block_index: 0,
                    }),
                    post_ops: [Call([])],
                })),
            ],
        },
    })),
    post_ops: [Call([])],
}));
