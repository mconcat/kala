use std::thread::scope;

use utils::{FxMap, Map, VectorMap};

use crate::{Statement, CaptureDeclaration, ParameterDeclaration, Expr, LocalDeclaration, CallPostOp, VariableCell, PropDef, Function, DeclarationIndex};

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
        $($stmt:tt;)*
    }) => {
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
    };

    (($($param:tt),*) {
        $($stmt:expr;)*
    }) => {
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
    };

    (($($param:tt),*) => $ret:expr) => {
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
    }
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

pub fn scope_function(function: &mut Function) {
    let mut map = FxMap::new();
    let mut captures = VectorMap::new();

    for parameter in function.parameters {
        match parameter {
            ParameterDeclaration::Variable { name } => {
                map.insert(&name, VariableCell::initialized(name, DeclarationIndex::Parameter(0), vec![]));
            },
            ParameterDeclaration::Pattern { pattern } => {
                pattern.visit(DeclarationIndex::Parameter(0), |name, property_access| {
                    map.insert(&name, VariableCell::initialized(name, DeclarationIndex::Parameter(0), property_access));
                });
            },
            ParameterDeclaration::Optional { name, default } => {
                map.insert(&name, VariableCell::initialized(name, DeclarationIndex::Parameter(0), vec![]));
            },
        }
    }

    for local in function.locals {
        match local {
            LocalDeclaration::Const { pattern, value } => {
                pattern.visit(DeclarationIndex::Local(0), |name, property_access| {
                    map.insert(&name, VariableCell::initialized(name, DeclarationIndex::Local(0), property_access));
                });
            },
            LocalDeclaration::Function { function } => {
                if let Some(name) = function.name.get_name() {
                    map.insert(&name, VariableCell::initialized(name.clone(), DeclarationIndex::Local(0), vec![]));
                }
            },
            LocalDeclaration::Let { pattern, value } => {
                pattern.visit(DeclarationIndex::Local(0), |name, property_access| {
                    map.insert(&name, VariableCell::initialized(name, DeclarationIndex::Local(0), property_access));
                });
            },
        }
    }

    for stmt in &mut function.statements.statements {
        scope_statement(stmt, &map, &mut captures);
    }

    function.captures = captures.drain().map(|(name, _)| {
        CaptureDeclaration::Local {
            name,
            variable: map.get(&name).unwrap().clone(),
        }
    }).collect();

}

pub fn scope_statement(stmt: &mut Statement, decls: &FxMap<VariableCell>, captures: &mut VectorMap<()>) {
    match stmt {
        Statement::Block(s) => s.statements.into_iter().for_each(|mut x| scope_statement(&mut x, map)),
        Statement::ExprStatement(s) => scope_expr(&mut *s, map),
        Statement::IfStatement(s) => {
            scope_expr(&mut s.condition, map);
            scope_expr(&mut s.condition, map);

        },
        Statement::Return(s) => scope_expr(&mut *s, map),
        Statement::Throw(s) => scope_expr(&mut *s, map),
        Statement::WhileStatement(s) => {
            scope_expr(&mut s.condition, map);
            s.body.statements.into_iter().for_each(|mut x| scope_statement(&mut x, map));
        }
        _ => {},
    }
}


pub fn scope_expr(expr: &mut Expr, map: &FxMap<VariableCell>) {
    match expr {
        Expr::Array(e) => e.0.into_iter().for_each(|x| scope_expr(&mut x, map)),
        Expr::Assignment(e) => {
            scope_lvalue(&mut e.1, map);
            scope_expr(&mut e.2, map);
        },
        Expr::BinaryExpr(e) => {
            scope_expr(&mut e.1, map);
            scope_expr(&mut e.2, map);
        },
        Expr::CallExpr(e) => {
            scope_expr(&mut e.expr, map);
            for op in e.post_ops {
                match op {
                    CallPostOp::Index(e) => scope_expr(&mut e, map),
                    CallPostOp::Call(es) => es.into_iter().for_each(|e| scope_expr(&mut e, map)),
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
                        let upper = map.get(&name).unwrap();
                        *variable = *upper;
                    },
                    CaptureDeclaration::Global { name } => unimplemented!("global")
                }
            }
        },
        Expr::ParenedExpr(e) => scope_expr(&mut *e, map),
        Expr::Record(e) => {
            for prop in e.0 {
                match prop {
                    PropDef::KeyValue(k, v) => {
                        
                    }
                }
            }
        },
        Expr::Spread(e) => scope_expr(&mut *e, map),
        Expr::UnaryExpr(e) => scope_expr(&mut e.expr, map),
        Expr::Variable(v) => {
            let upper = map.get(&v.name).unwrap();
            **v = *upper;
        }
    }
}