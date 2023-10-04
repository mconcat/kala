use std::thread::scope;

use utils::{FxMap, Map, VectorMap};

use crate::{Statement, CaptureDeclaration, ParameterDeclaration, Expr, LocalDeclaration, CallPostOp, VariableCell, PropDef, Function, DeclarationIndex, Pattern, PatternVisitor};

// - The function first iterates over the body of the function, and collects all the local declarations
// - Then, it iterates over the body again, and all the variables occuring inside the function body are bound to either one of local / capture / parameter
// - For any function declaration appearing inside, the list of capture variables are also bound to the function

// function_parameter_pattern: tt -> ParameterDeclaration
macro_rules! pattern {
    // Identifier
    ($param:ident) => {
        ParameterDeclaration::Variable {
            name: SharedString::from_str(stringify!($param)),
        }
    };

    // Array pattern
    ([$($elem:tt),*]) => {
        ParameterDeclaration::Pattern {
            pattern: Pattern::ArrayPattern(Box::new(ArrayPattern(patterns!($($elem),*)))),
        }
    };

    // Record pattern\
    // TODO
    /* 
    ({ $($prop:tt),* }) => {
        ParameterDeclaration::Pattern {
            pattern: Pattern::RecordPattern(Box::new(RecordPattern(record_property_patterns!($($prop),*)))),
        }
    };
    */

    // Optional
    ($param:ident = $default:expr) => {
        ParameterDeclaration::Optional {
            name: SharedString::from_str(stringify!($param)),
            default: $default,
        }
    };

    // TODO
/* 
    // Rest pattern
    (...$param:ident) => {
        ParameterDeclaration::Pattern {
            pattern: Pattern::Rest(Box::new(Pattern::Variable(Box::new(VariableCell::uninitialized(SharedString::from_str(stringify!($param))))))),
        }
    }
    */
}

#[macro_export]
macro_rules! patterns {
    // entry point
    ($first:tt) => {
        vec![pattern!($first)]
    };

    ($first:tt, $($rest:tt),*) => {
        patterns!(internal, [$($first),*], $($rest),*)
    };

    // Base case
    (internal, [$($pattern:expr),*]) => {
        vec![$($pattern),*]
    };

    (internal, [$($pattern:expr),*], $param:tt, $($rest:tt),*) => {
        patterns!([$($pattern),*,pattern!($param)], $($rest),*)
    };
}

// function_body: tt;* -> (Vec<LocalDeclaration>, Vec<Statement>)
#[macro_export]
macro_rules! function_body {
    // base case
    (internal, $locallen:expr, [$($local:expr),*,], [$($statement:expr),*,],) => {
        (vec![$($local),*], vec![$($statement),*])
    };

    // local declaration
    (internal, $locallen:expr, [$($local:expr),*], [$($statement:expr),*], let $pattern:expr => $localdecl:expr; $($rest:tt;)*) => {
        function_body!(internal, $locallen+1, [$($local),*, LocalDeclaration::Let{pattern: $pattern, value: Some($localdecl)}], [$($statement),*,Statement::LocalDeclaration(Box::new($locallen))], $($rest;)*)
    };

    // local declaration without init
    (internal, $locallen:expr, [$($local:expr),*], [$($statement:expr),*], let $pattern:expr; $($rest:tt;)*) => {
        function_body!(internal, $locallen+1, [$($local),*, LocalDeclaration::Let{pattern: $pattern, value: None}], [$($statement),*,Statement::LocalDeclaration(Box::new($locallen))], $($rest;)*)
    };

    // const declaration
    (internal, $locallen:expr, [$($local:expr),*], [$($statement:expr),*], const $pattern:expr => $localdecl:expr; $($rest:tt;)*) => {
        function_body!(internal, $locallen+1, [$($local),*, LocalDeclaration::Const{pattern: $pattern, value: Some($localdecl)}], [$($statement),*,Statement::LocalDeclaration(Box::new($locallen))], $($rest;)*)
    };

    // const declaration without init
    (internal, $locallen:expr, [$($local:expr),*], [$($statement:expr),*], const $pattern:expr; $($rest:tt;)*) => {
        function_body!(internal, $locallen+1, [$($local),*, LocalDeclaration::Const{pattern: $pattern, value: None}], [$($statement),*,Statement::LocalDeclaration(Box::new($locallen))], $($rest;)*)
    };

    // function declaration
    (internal, $locallen:expr, [$($local:expr),*], [$($statement:expr),*], function $name:ident $($param:tt),* { $($stmt:tt;)* }; $($rest:tt;)*) => {
        function_body!(internal, $locallen+1, [$($local),*, LocalDeclaration::Function{function: function!($name $($param),* { $($stmt;)* })}], [$($statement),*,Statement::LocalDeclaration(Box::new($locallen))], $($rest;)*)
    };

    // statement
    (internal, $locallen:expr, [$($local:expr),*], [$($statement:expr),*], $stmt:tt; $($rest:tt;)*) => {
        function_body!(internal, $locallen, [$($local),*], [$($statement),*, $stmt], $($rest;)*)
    };

    // entry point
    ($($stmt:tt;)*) => {
        function_body!(internal, 0, [], [], $($stmt;)*)
    };

    () => {
        function_body!(internal, 0, [], [],)
    };
}

#[macro_export]
macro_rules! function {    
    ($name:ident ($($param:tt),*) {
        $($stmt:expr;)*
    }) => {{
        let parameters = patterns!($($param),*);
        let (locals, statements) = function_body!($($stmt;)*);

        let function = Function {
            name: FunctionName::Named(SharedString::from_str(stringify!($name))),
            captures: vec![], // to be filled later
            parameters,
            locals,
            statements: Block { statements },
        };

        scope_function(&mut function);
    
        // at this point, function.captures is filled with all the capture declarations(without initialization)

        Expr::Function(Box::new(function))
    }};

    (($($param:tt),*) {
        $($stmt:expr;)*
    }) => {{
        let parameters = patterns!($($param),*);
        let (locals, statements) = function_body!($($stmt;)*);

        let function = Function {
            name: FunctionName::Anonymous,
            captures: vec![], // to be filled later
            parameters,
            locals,
            statements: Block { statements },
        };

        scope_function(&mut function);
    
        // at this point, function.captures is filled with all the capture declarations(without initialization)

        Expr::Function(Box::new(function)) 
    }};

    (($($param:tt),*) => $ret:expr) => {{
        let parameters = patterns!($($param),*);

        let function = Function {
            name: FunctionName::Anonymous,
            captures: vec![], // to be filled later
            parameters,
            locals: vec![],
            statements: Block { Statement::Return(Box::new( $ret )) },
        };

        scope_function(&mut function);
    
        // at this point, function.captures is filled with all the capture declarations(without initialization)

        Expr::Function(Box::new(function))  
    }}
}
/* 
#[macro_export]
macro_rules! function {
    

    (internal $name:ident [$($capture:ident),*] ($($param:ident),*) {
        $($localid:literal: $localdecl:expr;)*
    } {
        $($stmt:expr;)*
    }) => {
        let mut statements = vec![$($stmt),*];
        let captures = vec![$(CaptureDeclaration::uninitialized(SharedString::from_str(stringify!($capture)))),*];
        let parameters = vec![$(ParameterDeclaration::Variable{name: SharedString::from_str(stringify!($param))}),*];
        for stmt in statements.into_iter() {
            scope_statement(&mut stmt, captures, parameters);
        }

        Expr::Function(Box::new(Function {
            name: FunctionName::Named(SharedString::from_str(stringify!($name))),
            captures,
            parameters,
            locals: vec![$($localdecl),*],
            statements: Block{ statements },
        }))
    };

    ($name:ident [$($capture:ident),*] ($($param:ident),*) {
        $($localid:literal: $localdecl:expr;)*
    } {
        $($stmt:expr;)*
    }) => {
        function!(internal $name [$($capture),*] ($($param),*) {
            $($localid: $localdecl;)*
        } {
            $($stmt;)*
        })
    };

    ($name:ident [$($capture:ident),*] ($($param:ident),*) {
        $($stmt:expr;)*
    }) => {
        function!($name [$($capture),*] ($($param),*) {
        } {
            $($stmt;)*
        })
    };

    ($name:ident ($($param:ident),*) { $($stmt:expr;)* }) => {
        function!($name [] ($($param),*) {
        } {
            $($stmt;)*
        })
    }
}
*/
#[macro_export]
macro_rules! ret {
    ($expr:expr) => {
        Statement::Return(Box::new($expr.into()))
    };

    () => {
        Statement::ReturnEmpty()
    };
}

pub struct ScopingVisitor {
    pub map: FxMap<VariableCell>,
}

impl PatternVisitor for ScopingVisitor {
    fn visit(&mut self, index: DeclarationIndex, name: utils::SharedString, property_access: Vec<crate::PropertyAccess>) -> Option<()> {
        self.map.insert(name.clone(), VariableCell::initialized(name.clone(), index, property_access));
        Some(())
    }
}

pub fn scope_function(function: &mut Function) {
    let mut visitor = ScopingVisitor {
        map: FxMap::new(),
    };
    
    let mut captures = VectorMap::new();

    for (i, parameter) in function.parameters.clone().into_iter().enumerate() {
        match parameter {
            ParameterDeclaration::Variable { name } => {
                visitor.map.insert(name.clone(), VariableCell::initialized(name.clone(), DeclarationIndex::Parameter(i.try_into().unwrap()), vec![]));
            },
            ParameterDeclaration::Pattern { pattern } => {
                pattern.visit(DeclarationIndex::Parameter(i.try_into().unwrap()), &mut visitor);
            },
            ParameterDeclaration::Optional { name, default } => {
                visitor.map.insert(name.clone(), VariableCell::initialized(name.clone(), DeclarationIndex::Parameter(i.try_into().unwrap()), vec![]));
            },
        }
    }

    for (i, local) in function.locals.clone().into_iter().enumerate() {
        match local {
            LocalDeclaration::Const { pattern, value } => {
                pattern.visit(DeclarationIndex::Local(i.try_into().unwrap()), &mut visitor);
            },
            LocalDeclaration::Function { function } => {
                if let Some(name) = function.name.get_name() {
                    visitor.map.insert(name.clone(), VariableCell::initialized(name.clone(), DeclarationIndex::Local(i.try_into().unwrap()), vec![]));
                }
            },
            LocalDeclaration::Let { pattern, value } => {
                pattern.visit(DeclarationIndex::Local(i.try_into().unwrap()), &mut visitor);
            },
        }
    }

    for stmt in &mut function.statements.statements {
        scope_statement(stmt, &mut visitor.map, &mut captures);
    }

    function.captures = captures.drain().map(|(name, _)| {
        CaptureDeclaration::Local {
            name: name.clone(),
            variable: visitor.map.get(name).unwrap().clone(),
        }
    }).collect();

}

pub fn scope_statement(stmt: &mut Statement, decls: &mut FxMap<VariableCell>, captures: &mut VectorMap<()>) {
    match stmt {
        Statement::Block(s) => s.statements.clone().into_iter().for_each(|mut x| scope_statement(&mut x, decls, captures)),
        Statement::ExprStatement(s) => scope_expr(&mut *s, decls),
        Statement::IfStatement(s) => {
            scope_expr(&mut s.condition, decls);
            scope_expr(&mut s.condition, decls);

        },
        Statement::Return(s) => scope_expr(&mut *s, decls),
        Statement::Throw(s) => scope_expr(&mut *s, decls),
        Statement::WhileStatement(s) => {
            scope_expr(&mut s.condition, decls);
            s.body.statements.clone().into_iter().for_each(|mut x| scope_statement(&mut x, decls, captures));
        }
        _ => {},
    }
}

pub fn scope_expr(expr: &mut Expr, map: &mut FxMap<VariableCell>) {
    match expr {
        Expr::Array(e) => e.0.clone().into_iter().for_each(|mut x| scope_expr(&mut x, map)),
        Expr::Assignment(e) => {
            unimplemented!("assignment");
            //scope_lvalue(&mut e.1, map);
            scope_expr(&mut e.2, map);
        },
        Expr::BinaryExpr(e) => {
            scope_expr(&mut e.1, map);
            scope_expr(&mut e.2, map);
        },
        Expr::CallExpr(e) => {
            scope_expr(&mut e.expr, map);
            for op in &mut e.post_ops {
                match op {
                    CallPostOp::Index(e) => scope_expr(e, map),
                    CallPostOp::Call(es) => es.into_iter().for_each(|e| scope_expr(e, map)),
                    CallPostOp::Member(_) => {}
                }
            }
        },
        Expr::CondExpr(e) => {
            scope_expr(&mut e.0, map);
            scope_expr(&mut e.1, map); 
            scope_expr(&mut e.2, map);
        },
        Expr::DataLiteral(_) => {},
        Expr::Function(f) => {
            // Assume that the inteternal scoping is already done
            // Just bind the capture declarations to this function
            for capture in &mut f.captures {
                match capture {
                    CaptureDeclaration::Local { name, variable } => {
                        let upper = map.get(name.clone()).unwrap();
                        *variable = upper.clone();
                    },
                    CaptureDeclaration::Global { name } => unimplemented!("global")
                }
            }
        },
        Expr::ParenedExpr(e) => scope_expr(&mut *e, map),
        Expr::Record(e) => {
            unimplemented!("record");
        },
        Expr::Spread(e) => scope_expr(&mut *e, map),
        Expr::UnaryExpr(e) => scope_expr(&mut e.expr, map),
        Expr::Variable(v) => {
            let upper = map.get(v.name.clone()).unwrap();
            **v = upper.clone();
        }
    }
}