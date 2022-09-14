use core::panic;

use swc_ecma_ast as ast;

use crate::pattern::Pattern;
use crate::common::*;

///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
/// NodeF

pub trait NodeF: Sized {
    type Identifier: From<swc_ecma_ast::Ident> + From<swc_ecma_ast::PropName>;
    type Variable;

    type Statement: From<swc_ecma_ast::Stmt> + From<Statement<Self>>;
    type Block: From<swc_ecma_ast::BlockStmt> + From<BlockStatement<Self>>;
    type Expression: From<swc_ecma_ast::Expr> + From<Expression<Self>>;
    type Function: From<swc_ecma_ast::Function> + From<FunctionExpression<Self>>;
}

pub struct LexicalNodeF;

impl NodeF for LexicalNodeF {
    type Identifier = Identifier;
    type Variable = Identifier;

    type Statement = Statement<Self>;
    type Block = BlockStatement<Self>;
    type Expression = Expression<Self>;
    type Function = FunctionExpression<Self>;
}

///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
/// Module

////////////////////////////////////////////////////////////////////////
/// Type definitions

pub enum Program<F: NodeF> {
//    Module(Module),
    Script(Script<F>),
}

pub struct Script<F: NodeF> {
    pub body: Vec<F::Statement>
}

////////////////////////////////////////////////////////////////////////
/// SWC AST integration

impl<F: NodeF> From<ast::Script> for Script<F> {
    fn from(script: ast::Script) -> Self {
        Script {
            body: script.body.into_iter().map(|stmt| stmt.into()).collect(),
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
/// Statements

////////////////////////////////////////////////////////////////////////
/// Type definitions

pub enum Statement<F: NodeF> {
    // Declarations
    VariableDeclaration(Box<VariableDeclaration<F>>),
    FunctionDeclaration(Box<FunctionDeclaration<F>>),

    // Block
    Block(Box<F::Block>),

    // If
    If(Box<IfStatement<F>>),

    // Breakable Statements
    For(Box<ForStatement<F>>),
    ForOf(Box<ForOfStatement<F>>),
    While(Box<WhileStatement<F>>),
    Switch(Box<SwitchStatement<F>>),

    // Try-catch
    Try(Box<TryStatement<F>>),

    // Terminators
    Break(Box<BreakStatement>),
    Continue(Box<ContinueStatement>),
    Return(Box<ReturnStatement<F>>),
    Throw(Box<ThrowStatement<F>>),

    // Expression
    Expression(Box<ExpressionStatement<F>>),
}

////////////////////////////////////////////////////////////////////////
// Declaration statements

pub struct VariableDeclarator<F: NodeF> {
    pub binding: Pattern,
    pub init: Option<F::Expression>,
}

pub struct VariableDeclaration<F: NodeF> {
    pub kind: DeclarationKind,
    pub declarators: Vec<VariableDeclarator<F>>,
}

pub struct FunctionDeclaration<F: NodeF> {
    pub function: F::Function
}

////////////////////////////////////////////////////////////////////////
/// Control flow

pub struct BlockStatement<F: NodeF> {
    pub body: Vec<F::Statement>,
}

pub struct IfStatement<F: NodeF> {
    pub test: F::Expression,
    pub consequent: F::Statement,
    pub alternate: Option<F::Statement>,
}

pub struct ForStatement<F: NodeF> {
    pub kind: DeclarationKind,
    pub init: Option<VariableDeclaration<F>>,
    pub test: Option<F::Expression>,
    pub update: Option<F::Expression>,
    pub body: F::Statement,
}

pub struct ForOfStatement<F: NodeF> {
    pub kind: DeclarationKind,
    pub decl: VariableDeclarator<F>,
    pub body: F::Statement,
}

pub struct WhileStatement<F: NodeF> {
    pub test: F::Expression,
    pub body: F::Statement,
}

pub struct SwitchStatement<F: NodeF> {
    pub discriminant: F::Expression,
    pub cases: Vec<SwitchCase<F>>,
}

pub struct SwitchCase<F: NodeF> {
    pub cases: Vec<CaseLabel<F>>,
    pub body: F::Block, // must end with a terminator
}

pub enum CaseLabel<F: NodeF> {
    Test(F::Expression),
    Default,
}

pub struct TryStatement<F: NodeF> {
    pub block: BlockStatement<F>,
    pub handler: Option<(Pattern, BlockStatement<F>)>,
    pub finalizer: Option<BlockStatement<F>>,
}

////////////////////////////////////////////////////////////////////////
// Terminators

pub enum Terminator<F: NodeF> {
    Break(BreakStatement),
    Continue(ContinueStatement),
    Return(ReturnStatement<F>),
    Throw(ThrowStatement<F>),
}

pub struct BreakStatement{}

pub struct ContinueStatement{}

pub struct ReturnStatement<F: NodeF> {
    pub argument: Option<F::Expression>
}

pub struct ThrowStatement<F: NodeF> {
    pub argument: F::Expression
}

////////////////////////////////////////////////////////////////////////
/// Expression Statements

pub struct ExpressionStatement<F: NodeF> {
    pub expression: F::Expression
}

////////////////////////////////////////////////////////////////////////
/// SWC integration

impl<F: NodeF> From<ast::Stmt> for Statement<F> {
    fn from(stmt: ast::Stmt) -> Self {
        match stmt {
            ast::Stmt::Decl(decl) => match decl {
                ast::Decl::Var(decl) => Statement::VariableDeclaration(Box::new(decl.into())),
                ast::Decl::Fn(decl) => Statement::FunctionDeclaration(Box::new(decl.into())),
                //ast::Decl::TsInterface(decl) => Statement::InterfaceDeclaration(decl.into()),
                //ast::Decl::TsTypeAlias(decl) => Statement::TypeAliasDeclaration(decl.into()),
                //ast::Decl::TsEnum(decl) => Statement::EnumDeclaration(decl.into()),
                _ => unimplemented!(),
            },
            ast::Stmt::Block(stmt) => Statement::Block(Box::new(stmt.into())),
            ast::Stmt::If(stmt) => Statement::If(Box::new(stmt.into())),
            ast::Stmt::For(stmt) => Statement::For(Box::new(stmt.into())),
            ast::Stmt::ForOf(stmt) => Statement::ForOf(Box::new(stmt.into())),
            ast::Stmt::While(stmt) => Statement::While(Box::new(stmt.into())),
            ast::Stmt::Switch(stmt) => Statement::Switch(Box::new(stmt.into())),
            ast::Stmt::Try(stmt) => Statement::Try(Box::new(stmt.into())),
            ast::Stmt::Break(stmt) => Statement::Break(Box::new(stmt.into())),
            ast::Stmt::Continue(stmt) => Statement::Continue(Box::new(stmt.into())),
            ast::Stmt::Return(stmt) => Statement::Return(Box::new(stmt.into())),
            ast::Stmt::Throw(stmt) => Statement::Throw(Box::new(stmt.into())),
            ast::Stmt::Expr(stmt) => Statement::Expression(Box::new(stmt.into())),
            _ => unimplemented!(),
        }
    }
}



impl<F: NodeF> From<ast::VarDecl> for VariableDeclaration<F> {
    fn from(decl: ast::VarDecl) -> Self {
        VariableDeclaration {
            kind: match decl.kind {
                ast::VarDeclKind::Var => unimplemented!(),
                ast::VarDeclKind::Let => DeclarationKind::Let,
                ast::VarDeclKind::Const => DeclarationKind::Const,
            },
            declarators: decl.decls.into_iter().map(|decl| decl.into()).collect(),
        }
    }
}

impl<F: NodeF> From<ast::VarDeclarator> for VariableDeclarator<F> {
    fn from(decl: ast::VarDeclarator) -> Self {
        VariableDeclarator {
            binding: decl.name.into(),
            init: decl.init.map(|init| (*init).into()),
        }
    }
}

impl<F: NodeF> From<ast::FnDecl> for FunctionDeclaration<F> {
    fn from(decl: ast::FnDecl) -> Self {
        let mut function: FunctionExpression<F> = decl.function.into();
        function.name = Some(decl.ident.into());
        FunctionDeclaration { function: function.into() }
    }
}

impl<F: NodeF> From<ast::BlockStmt> for BlockStatement<F> {
    fn from(stmt: ast::BlockStmt) -> Self {
        BlockStatement {
            body: stmt.stmts.into_iter().map(|stmt| stmt.into()).collect(),
        }
    }
}

impl<F: NodeF> From<ast::IfStmt> for IfStatement<F> {
    fn from(stmt: ast::IfStmt) -> Self {
        IfStatement {
            test: (*stmt.test).into(),
            consequent: (*stmt.cons).into(),
            alternate: stmt.alt.map(|stmt| (*stmt).into()),
        }
    }
}

impl<F: NodeF> From<ast::ForStmt> for ForStatement<F> {
    fn from(stmt: ast::ForStmt) -> Self {
        /*
        ForStatement {
            init: stmt.init.map(|init| init.into()),
            test: stmt.test.map(|test| (*test).into()),
            update: stmt.update.map(|update| (*update).into()),
            body: stmt.body.into(),
        }
        */
        unimplemented!()
    }
}

impl<F: NodeF> From<ast::ForOfStmt> for ForOfStatement<F> {
    fn from(stmt: ast::ForOfStmt) -> Self {
        unimplemented!()
    }
}

impl<F: NodeF> From<ast::WhileStmt> for WhileStatement<F> {
    fn from(stmt: ast::WhileStmt) -> Self {
        WhileStatement {
            test: (*stmt.test).into(),
            body: (*stmt.body).into(),
        }
    }
}

impl<F: NodeF> From<ast::SwitchStmt> for SwitchStatement<F> {
    fn from(stmt: ast::SwitchStmt) -> Self {
        SwitchStatement {
            discriminant: (*stmt.discriminant).into(),
            cases: stmt.cases.into_iter().map(|case| case.into()).collect(),
        }
    }
}

impl<F: NodeF> From<ast::SwitchCase> for SwitchCase<F> {
    fn from(case: ast::SwitchCase) -> Self {
        /*
        SwitchCase {
            test: case.test.map(|test| Box::new(test.into())),
            consequent: case.cons.into_iter().map(|stmt| stmt.into()).collect(),
        }
        */
        unimplemented!()
    }
}

impl<F: NodeF> From<ast::TryStmt> for TryStatement<F> {
    fn from(stmt: ast::TryStmt) -> Self {
        /*
        TryStatement {
            block: stmt.block.into(),
            handler: stmt.handler.map(|handler| (handler.param)),
            finalizer: stmt.finalizer.map(|finalizer| finalizer.into()),
        }
        */
        unimplemented!()
    }
}

impl From<ast::BreakStmt> for BreakStatement {
    fn from(stmt: ast::BreakStmt) -> Self {
        BreakStatement {
            // TODO: label
        }
    }
}

impl From<ast::ContinueStmt> for ContinueStatement {
    fn from(stmt: ast::ContinueStmt) -> Self {
        ContinueStatement {
            // TODO: label
        }
    }
}

impl<F: NodeF> From<ast::ReturnStmt> for ReturnStatement<F> {
    fn from(stmt: ast::ReturnStmt) -> Self {
        ReturnStatement {
            argument: stmt.arg.map(|arg| (*arg).into()),
        }
    }
}

impl<F: NodeF> From<ast::ThrowStmt> for ThrowStatement<F> {
    fn from(stmt: ast::ThrowStmt) -> Self {
        ThrowStatement {
            argument: (*stmt.arg).into(),
        }
    }
}

impl<F: NodeF> From<ast::ExprStmt> for ExpressionStatement<F> {
    fn from(stmt: ast::ExprStmt) -> Self {
        ExpressionStatement {
            expression: (*stmt.expr).into(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
/// Expressions

///////////////////////////////////////////////////////////////////////////////
/// Type definitions

pub enum Expression<F: NodeF> {
    // Literals
    Literal(Box<Literal>),
    Array(Box<ArrayExpression<F>>),
    Object(Box<ObjectExpression<F>>),
    Function(Box<FunctionExpression<F>>),
    ArrowFunction(Box<ArrowFunctionExpression<F>>),

    // Operations
    Unary(Box<UnaryExpression<F>>),
    Binary(Box<BinaryExpression<F>>),
    Logical(Box<LogicalExpression<F>>),
    Conditional(Box<ConditionalExpression<F>>),
    Update(Box<UpdateExpression<F>>),

    // Variable Expressions
    Variable(Box<VariableExpression<F>>),
    Assignment(Box<AssignmentExpression<F>>),
    Member(Box<MemberExpression<F>>),
    Call(Box<CallExpression<F>>),

    // Parenthesized
    Parenthesized(Box<ParenthesizedExpression<F>>),
}

pub struct ArrayExpression<F: NodeF> {
    pub elements: Vec<ParameterElement<F>>,
}

pub enum ParameterElement<F: NodeF> {
    Parameter(F::Expression),
    Spread(F::Expression),
}

pub struct ObjectExpression<F: NodeF> {
    pub properties: Vec<ObjectElement<F>>,
}

pub enum ObjectElement<F: NodeF> {
    KeyValue(F::Identifier, F::Expression),
    Shorthand(F::Identifier),
    Getter(F::Identifier, F::Block), // Must return a value
    Setter(F::Identifier, F::Identifier, F::Block), // Must not return a value
    Method(F::Function),
    Spread(F::Expression),
}

pub struct FunctionExpression<F: NodeF> {
    pub name: Option<F::Identifier>,
    pub params: Vec<Pattern>,
    pub body: F::Block,
}

pub struct ArrowFunctionExpression<F: NodeF> {
    pub params: Vec<Pattern>,
    pub body: ArrowFunctionBody<F>
}

pub enum ArrowFunctionBody<F: NodeF> {
    Block(F::Block),
    Expression(F::Expression),
}

////////////////////////////////////////////////////////////////////////
/// Operations

pub struct UnaryExpression<F: NodeF> {
    pub operator: UnaryOperator,
    pub argument: F::Expression,
}

pub enum UnaryOperator {
    Void,
    TypeOf,
    Plus,
    Minus,
    Bang,
    Tilde,
}

pub struct BinaryExpression<F: NodeF> {
    pub operator: BinaryOperator,
    pub left: F::Expression,
    pub right: F::Expression,
}

pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    BitAnd,
    BitOr,
    BitXor,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    StrictEqual,
    StrictNotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
}

pub struct UpdateExpression<F: NodeF> {
    pub operator: UpdateOperator,
    pub argument: F::Expression,
    pub prefix: bool,
}

pub enum UpdateOperator {
    Increment,
    Decrement,
}

pub struct LogicalExpression<F: NodeF> {
    pub operator: LogicalOperator,
    pub left: F::Expression,
    pub right: F::Expression,
}

pub enum LogicalOperator {
    And,
    Or,
    Coalesce,
}

pub struct ConditionalExpression<F: NodeF> {
    pub test: F::Expression,
    pub consequent: F::Expression,
    pub alternate: F::Expression,
}

////////////////////////////////////////////////////////////////////////
/// 
/// Variable Expressions

pub struct VariableExpression<F: NodeF> {
    pub name: F::Identifier
}

pub struct AssignmentExpression<F: NodeF> {
    pub operator: AssignmentOperator,
    pub left: LValue<F>,
    pub right: F::Expression,
}

pub enum LValue<F: NodeF> {
    Variable(F::Identifier),
    Member(MemberExpression<F>),
}

pub enum AssignmentOperator {
    Assign,
    /*
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    PowAssign,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
    LeftShiftAssign,
    RightShiftAssign,
    UnsignedRightShiftAssign,
    */
}

pub struct MemberExpression<F: NodeF> {
    pub object: F::Expression,
    pub property: Member<F>,
}

pub enum Member<F: NodeF> {
    Computed(F::Expression),
    Property(F::Identifier),
}

pub struct CallExpression<F: NodeF> {
    pub callee: F::Expression,
    pub arguments: Vec<ParameterElement<F>>,
}

pub struct ParenthesizedExpression<F: NodeF> {
    pub expression: F::Expression
}

////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////
/// SWC integration

impl<F: NodeF> From<ast::Expr> for Expression<F> {
    fn from(expr: ast::Expr) -> Self {
        match expr {
            ast::Expr::Lit(lit) => Expression::Literal(Box::new(lit.into())),
            ast::Expr::Array(array) => Expression::Array(Box::new(array.into())),
            ast::Expr::Object(object) => Expression::Object(Box::new(object.into())),
            ast::Expr::Fn(fn_expr) => Expression::Function(Box::new(fn_expr.into())),
            ast::Expr::Arrow(arrow) => Expression::ArrowFunction(Box::new(arrow.into())),

            ast::Expr::Unary(unary) => Expression::Unary(Box::new(unary.into())),
            ast::Expr::Update(update) => Expression::Update(Box::new(update.into())),
            ast::Expr::Bin(bin) => match bin.op {
                ast::BinaryOp::LogicalAnd |
                ast::BinaryOp::LogicalOr |
                ast::BinaryOp::NullishCoalescing => Expression::Logical(Box::new(bin.into())),
                _ => Expression::Binary(Box::new(bin.into())),
            }
            ast::Expr::Cond(cond) => Expression::Conditional(Box::new(cond.into())),

            ast::Expr::Ident(ident) => Expression::Variable(Box::new(ident.into())),
            ast::Expr::Assign(assign) => Expression::Assignment(Box::new(assign.into())),
            ast::Expr::Member(member) => Expression::Member(Box::new(member.into())),
            ast::Expr::Call(call) => Expression::Call(Box::new(call.into())),

            ast::Expr::Paren(paren) => Expression::Parenthesized(Box::new(paren.into())),

            _ => unimplemented!(),
        }
    }
}


impl<F: NodeF> From<ast::ArrayLit> for ArrayExpression<F> {
    fn from(array: ast::ArrayLit) -> Self {
        ArrayExpression {
            elements: array.elems.into_iter().map(|elem| elem.map(|elem| elem.into()).unwrap_or(ParameterElement::Parameter(Expression::Literal(Box::new(Literal::Undefined)).into()))).collect(),
        }
    }
}


impl<F: NodeF> From<ast::ObjectLit> for ObjectExpression<F> {
    fn from(object: ast::ObjectLit) -> Self {
        ObjectExpression {
            properties: object.props.into_iter().map(|prop| prop.into()).collect(),
        }
    }
}

impl<F: NodeF> From<ast::PropOrSpread> for ObjectElement<F> {
    fn from(prop: ast::PropOrSpread) -> Self {
        match prop {
            ast::PropOrSpread::Prop(prop) => (*prop).into(), 
            ast::PropOrSpread::Spread(spread) => ObjectElement::Spread((*spread.expr).into())
        }
    }
}

impl<F: NodeF> From<ast::Prop> for ObjectElement<F> {
    fn from(prop: ast::Prop) -> Self {
        match prop {
            ast::Prop::Shorthand(ident) => ObjectElement::Shorthand(ident.into()),
            ast::Prop::KeyValue(key_value) => ObjectElement::KeyValue(key_value.key.into(), (*key_value.value).into()),
            ast::Prop::Getter(getter) => ObjectElement::Getter(getter.key.into(), getter.body.unwrap().into()),
            ast::Prop::Setter(setter) => match setter.param {
                ast::Pat::Ident(ident) => ObjectElement::Setter(setter.key.into(), ident.id.into(), setter.body.unwrap().into()),
                _ => unimplemented!(),
            },
            ast::Prop::Method(method) => ObjectElement::Method(method.function.into()),
            _ => unimplemented!(),
        }
    }
}

impl From<ast::PropName> for Identifier {
    fn from(prop_name: ast::PropName) -> Self {
        match prop_name {
            ast::PropName::Ident(ident) => ident.into(),
            _ => unimplemented!(),
        }
    }
}

impl<F: NodeF> From<ast::FnExpr> for FunctionExpression<F> {
    fn from(fn_expr: ast::FnExpr) -> Self {
        FunctionExpression {
            name: fn_expr.ident.map(|ident| ident.into()),
            params: fn_expr.function.params.into_iter().map(|param| param.into()).collect(),
            body: fn_expr.function.body.expect("function body is required").into(),
        }
    }
}

impl<F: NodeF> From<ast::Function> for FunctionExpression<F> {
    fn from(function: ast::Function) -> Self {
        FunctionExpression {
            name: None,
            params: function.params.into_iter().map(|param| param.into()).collect(),
            body: function.body.map(|x| x.into()).unwrap_or(BlockStatement{body: vec![]}.into()),
        }
    }
}

impl<F: NodeF> From<ast::ArrowExpr> for ArrowFunctionExpression<F> {
    fn from(arrow: ast::ArrowExpr) -> Self {
        if arrow.is_async || arrow.is_generator {
            unimplemented!()
        }
        ArrowFunctionExpression {
            params: arrow.params.into_iter().map(|param| param.into()).collect(),
            body: match arrow.body {
                ast::BlockStmtOrExpr::BlockStmt(block) => ArrowFunctionBody::Block(block.into()),
                ast::BlockStmtOrExpr::Expr(expr) => ArrowFunctionBody::Expression((*expr).into()),
            },
        }
    }
}

impl<F: NodeF> From<ast::UnaryExpr> for UnaryExpression<F> {
    fn from(unary: ast::UnaryExpr) -> Self {
        UnaryExpression {
            operator: unary.op.into(),
            argument: (*unary.arg).into(),
        }
    }
}

impl From<ast::UnaryOp> for UnaryOperator {
    fn from(op: ast::UnaryOp) -> Self {
        match op {
            ast::UnaryOp::Minus => UnaryOperator::Minus,
            ast::UnaryOp::Plus => UnaryOperator::Plus,
            ast::UnaryOp::Bang => UnaryOperator::Bang,
            ast::UnaryOp::Tilde => UnaryOperator::Tilde,
            ast::UnaryOp::TypeOf => UnaryOperator::TypeOf,
            ast::UnaryOp::Void => UnaryOperator::Void,
            ast::UnaryOp::Delete => panic!("Delete operator is not supported"),
        }
    }
}

impl<F: NodeF> From<ast::UpdateExpr> for UpdateExpression<F> {
    fn from(update: ast::UpdateExpr) -> Self {
        UpdateExpression {
            operator: update.op.into(),
            argument: (*update.arg).into(),
            prefix: update.prefix,
        }
    }
}

impl From<ast::UpdateOp> for UpdateOperator {
    fn from(op: ast::UpdateOp) -> Self {
        match op {
            ast::UpdateOp::PlusPlus => UpdateOperator::Increment,
            ast::UpdateOp::MinusMinus => UpdateOperator::Decrement,
        }
    }
}

impl<F: NodeF> From<ast::BinExpr> for BinaryExpression<F> {
    fn from(bin: ast::BinExpr) -> Self {
        BinaryExpression {
            operator: match bin.op {
                ast::BinaryOp::Add => BinaryOperator::Add,
                ast::BinaryOp::Sub => BinaryOperator::Sub,
                ast::BinaryOp::Mul => BinaryOperator::Mul,
                ast::BinaryOp::Div => BinaryOperator::Div,
                ast::BinaryOp::Mod => BinaryOperator::Mod,
                ast::BinaryOp::Exp => BinaryOperator::Pow,
                ast::BinaryOp::LShift => BinaryOperator::LeftShift,
                ast::BinaryOp::RShift => BinaryOperator::RightShift,
                ast::BinaryOp::ZeroFillRShift => BinaryOperator::UnsignedRightShift,
                ast::BinaryOp::BitOr => BinaryOperator::BitOr,
                ast::BinaryOp::BitXor => BinaryOperator::BitXor,
                ast::BinaryOp::BitAnd => BinaryOperator::BitAnd,
                ast::BinaryOp::EqEq => panic!("== is not supported"),
                ast::BinaryOp::NotEq => panic!("!= is not supported"),
                ast::BinaryOp::EqEqEq => BinaryOperator::StrictEqual,
                ast::BinaryOp::NotEqEq => BinaryOperator::StrictNotEqual,
                ast::BinaryOp::Lt => BinaryOperator::LessThan,
                ast::BinaryOp::LtEq => BinaryOperator::LessThanEqual,
                ast::BinaryOp::Gt => BinaryOperator::GreaterThan,
                ast::BinaryOp::GtEq => BinaryOperator::GreaterThanEqual,
                ast::BinaryOp::In => panic!("in is not supported"),
                ast::BinaryOp::InstanceOf => panic!("instanceof is not supported"),
                ast::BinaryOp::LogicalAnd => unreachable!("LogicalAnd is not BinaryExpression"),
                ast::BinaryOp::LogicalOr => unreachable!("LogicalOr is not BinaryExpression"),
                ast::BinaryOp::NullishCoalescing => unreachable!("NullishCoalescing is not BinaryExpression"),
            },
            left: (*bin.left).into(),
            right: (*bin.right).into(),
        }
    }
}

impl<F: NodeF> From<ast::BinExpr> for LogicalExpression<F> {
    fn from(bin: ast::BinExpr) -> Self {
        LogicalExpression {
            operator: match bin.op {
                ast::BinaryOp::LogicalOr => LogicalOperator::Or,
                ast::BinaryOp::LogicalAnd => LogicalOperator::And,
                ast::BinaryOp::NullishCoalescing => LogicalOperator::Coalesce,
                _ => unimplemented!(),
            },
            left: (*bin.left).into(),
            right: (*bin.right).into(),
        }
    }
}

impl<F: NodeF> From<ast::CondExpr> for ConditionalExpression<F> {
    fn from(cond: ast::CondExpr) -> Self {
        ConditionalExpression {
            test: (*cond.test).into(),
            consequent: (*cond.cons).into(),
            alternate: (*cond.alt).into(),
        }
    }
}

impl<F: NodeF> From<ast::Ident> for VariableExpression<F> {
    fn from(ident: ast::Ident) -> Self {
        VariableExpression { name: ident.into() }
    }
}

impl<F: NodeF> From<ast::AssignExpr> for AssignmentExpression<F> {
    fn from(assign: ast::AssignExpr) -> Self {
        AssignmentExpression {
            operator: match assign.op {
                ast::AssignOp::Assign => AssignmentOperator::Assign,
                //ast::AssignOp::AddAssign => AssignmentOperator::AddAssign,
                //ast::AssignOp::SubAssign => AssignmentOperator::SubAssign,
                //ast::AssignOp::MulAssign => AssignmentOperator::MulAssign,
                //ast::AssignOp::DivAssign => AssignmentOperator::DivAssign,
                //ast::AssignOp::ModAssign => AssignmentOperator::ModAssign,
                //ast::AssignOp::ShlAssign => AssignmentOperator::ShlAssign,
                //ast::AssignOp::ShrAssign => AssignmentOperator::ShrAssign,
                //ast::AssignOp::UShrAssign => AssignmentOperator::UShrAssign,
                //ast::AssignOp::BitOrAssign => AssignmentOperator::BitOrAssign,
                //ast::AssignOp::BitXorAssign => AssignmentOperator::BitXorAssign,
                //ast::AssignOp::BitAndAssign => AssignmentOperator::BitAndAssign,
                //ast::AssignOp::ExpAssign => AssignmentOperator::ExpAssign,
                _ => unimplemented!(),
            },
            left: assign.left.into(),
            right: (*assign.right).into(),
        }
    }
}

impl<F: NodeF> From<ast::PatOrExpr> for LValue<F> {
    fn from(pat: ast::PatOrExpr) -> Self {
        match pat {
            ast::PatOrExpr::Expr(expr) => match *expr {
                ast::Expr::Ident(ident) => LValue::Variable(ident.into()),
                ast::Expr::Member(member) => LValue::Member(member.into()),
                _ => unimplemented!(),
            },
            ast::PatOrExpr::Pat(_) => panic!("Pattern is not supported"),
        }
    }
}

impl<F: NodeF> From<ast::MemberExpr> for MemberExpression<F> {
    fn from(member: ast::MemberExpr) -> Self {
        MemberExpression {
            object: (*member.obj).into(),
            property: match member.prop {
                ast::MemberProp::Ident(ident) => Member::Property(ident.into()),
                ast::MemberProp::Computed(computed) => Member::Computed((*computed.expr).into()),
                ast::MemberProp::PrivateName(_) => unimplemented!(),
            },
        }
    }
}

impl<F: NodeF> From<ast::CallExpr> for CallExpression<F> {
    fn from(call: ast::CallExpr) -> Self {
        CallExpression {
            callee: match call.callee {
                ast::Callee::Expr(expr) => (*expr).into(),
                ast::Callee::Super(_) => panic!("super is not supported"), 
                ast::Callee::Import(_) => unimplemented!("import callee"),
            },
            arguments: call.args.into_iter().map(|arg| arg.into()).collect(),
        }
    }
}

impl<F: NodeF> From<ast::ExprOrSpread> for ParameterElement<F> {
    fn from(expr_or_spread: ast::ExprOrSpread) -> Self {
        if expr_or_spread.spread.is_some() {
            ParameterElement::Spread((*expr_or_spread.expr).into())
        } else {
            ParameterElement::Parameter((*expr_or_spread.expr).into())
        }
    }
}

impl<F: NodeF> From<ast::ParenExpr> for ParenthesizedExpression<F> {
    fn from(paren: ast::ParenExpr) -> Self {
        ParenthesizedExpression {
            expression: (*paren.expr).into(),
        }
    }
}