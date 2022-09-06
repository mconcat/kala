// Jessie AST definitions, with neccesary interpreter runtime metadata.

pub enum DeclarationKind {
    Let,
    Const,
}

pub struct Identifier {
    pub name: String,
}

////////////////////////////////////////////////////////////////////////
/// Module

pub enum Program {
//    Module(Module),
    Script(Script),
}

pub struct Script {
    pub body: Vec<Statement>
}

////////////////////////////////////////////////////////////////////////
/// Literals

pub struct NumberLiteral{
    pub value: i64,
}

pub struct StringLiteral {
    pub value: String,
}

pub struct BooleanLiteral {
    pub value: bool,
}

pub struct BigintLiteral {
    pub value: String,
}

pub enum Literal {
    Undefined,
    Null,
    Number(NumberLiteral),
    Boolean(BooleanLiteral),
    String(StringLiteral),
    Bigint(BigintLiteral),
}

////////////////////////////////////////////////////////////////////////
/// Patterns
///
/// Patterns are used in variable declarations, function parameters, and
/// destructuring assignments.

pub enum Pattern {
    Identifier(Identifier),
    Literal(Literal),
    Array(ArrayPattern),
    Object(ObjectPattern),
    Hole,
    Rest(Box<Pattern>),
    Optional(OptionalPattern),
}

pub struct OptionalPattern {
    pub binding: Identifier,
    pub default: Expression,
}

pub struct ArrayPattern {
    pub elements: Vec<Pattern>,
}

pub struct ObjectPattern {
    pub properties: Vec<PropertyPattern>,
}

pub enum PropertyPattern {
    KeyValue(Identifier, Pattern),
    Shorthand(Identifier),
    Optional(OptionalPattern),
    Rest(Pattern),
}

////////////////////////////////////////////////////////////////////////
/// Statements
/// 
/// Statements are the basic building blocks of a program. They are
/// executed in order, and can be nested.

pub enum Statement {
    // Declarations
    VariableDeclaration(Box<VariableDeclaration>),
    FunctionDeclaration(Box<FunctionDeclaration>),

    // Block
    Block(Box<BlockStatement>),

    // If
    If(Box<IfStatement>),

    // Breakable Statements
    For(Box<ForStatement>),
    ForOf(Box<ForOfStatement>),
    While(Box<WhileStatement>),
    Switch(Box<SwitchStatement>),

    // Try-catch
    Try(Box<TryStatement>),

    // Terminators
    Break(Box<BreakStatement>),
    Continue(Box<ContinueStatement>),
    Return(Box<ReturnStatement>),
    Throw(Box<ThrowStatement>),

    // Expression
    Expression(Box<ExpressionStatement>),
}

////////////////////////////////////////////////////////////////////////
// Declaration statements

pub struct VariableDeclarator {
    pub binding: Pattern,
    pub init: Option<Expression>,
}

pub struct VariableDeclaration {
    pub kind: DeclarationKind,
    pub declarators: Vec<VariableDeclarator>,
}

pub struct FunctionDeclaration {
    function: FunctionExpression
}

////////////////////////////////////////////////////////////////////////
/// Control flow

pub struct BlockStatement {
    pub body: Vec<Statement>,
}

pub struct IfStatement {
    pub test: Expression,
    pub consequent: Statement,
    pub alternate: Option<Statement>,
}

pub struct ForStatement {
    pub kind: DeclarationKind,
    pub init: Option<VariableDeclaration>,
    pub test: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Statement,
}

pub struct ForOfStatement {
    pub kind: DeclarationKind,
    pub decl: VariableDeclarator,
    pub body: Statement,
}

pub struct WhileStatement {
    pub test: Expression,
    pub body: Statement,
}

pub struct SwitchStatement {
    pub discriminant: Expression,
    pub cases: Vec<SwitchCase>,
}

pub struct SwitchCase {
    pub cases: Vec<CaseLabel>,
    pub body: BlockStatement, // must end with a terminator
}

pub enum CaseLabel {
    Test(Expression),
    Default,
}

pub struct TryStatement {
    pub block: BlockStatement,
    pub handler: Option<(Pattern, BlockStatement)>,
    pub finalizer: Option<BlockStatement>,
}

////////////////////////////////////////////////////////////////////////
// Terminators

pub enum Terminator {
    Break(BreakStatement),
    Continue(ContinueStatement),
    Return(ReturnStatement),
    Throw(ThrowStatement),
}

pub struct BreakStatement{}

pub struct ContinueStatement{}

pub struct ReturnStatement{
    pub argument: Option<Expression>
}

pub struct ThrowStatement{
    pub argument: Expression
}

////////////////////////////////////////////////////////////////////////
/// Expression Statements

pub struct ExpressionStatement {
    pub expression: Expression
}

////////////////////////////////////////////////////////////////////////
/// Expressions

pub enum Expression {
    // Literals
    Literal(Box<Literal>),
    Array(Box<ArrayExpression>),
    Object(Box<ObjectExpression>),
    Function(Box<FunctionExpression>),
    ArrowFunction(Box<ArrowFunctionExpression>),

    // Operations
    Unary(Box<UnaryExpression>),
    Binary(Box<BinaryExpression>),
    Logical(Box<LogicalExpression>),
    Conditional(Box<ConditionalExpression>),
    Update(Box<UpdateExpression>),

    // Variable Expressions
    Variable(Box<VariableExpression>),
    Assignment(Box<AssignmentExpression>),
    Member(Box<MemberExpression>),
    Call(Box<CallExpression>),

    // Parenthesized
    Parenthesized(Box<ParenthesizedExpression>),
}

pub struct ArrayExpression {
    pub elements: Vec<ParameterElement>,
}

pub enum ParameterElement {
    Parameter(Expression),
    Spread(Expression),
}

pub struct ObjectExpression {
    pub properties: Vec<ObjectElement>,
}

pub enum ObjectElement {
    KeyValue(Identifier, Expression),
    Shorthand(Identifier),
    Getter(Identifier, BlockStatement), // Must return a value
    Setter(Identifier, Pattern, BlockStatement), // Must not return a value
    Method(FunctionExpression),
    Spread(Expression),
}

pub struct FunctionExpression {
    pub name: Option<Identifier>,
    pub params: Vec<Pattern>,
    pub body: BlockStatement,
}

pub struct ArrowFunctionExpression {
    pub params: Vec<Pattern>,
    pub body: ArrowFunctionBody
}

pub enum ArrowFunctionBody {
    Block(BlockStatement),
    Expression(Expression),
}

////////////////////////////////////////////////////////////////////////
/// Operations

pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub argument: Expression,
}

pub enum UnaryOperator {
    Void,
    TypeOf,
    Plus,
    Minus,
    Bang,
    Tilde,
}

pub struct BinaryExpression {
    pub operator: BinaryOperator,
    pub left: Expression,
    pub right: Expression,
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

pub struct UpdateExpression {
    pub operator: UpdateOperator,
    pub argument: Expression,
    pub prefix: bool,
}

pub enum UpdateOperator {
    Increment,
    Decrement,
}

pub struct LogicalExpression {
    pub operator: LogicalOperator,
    pub left: Expression,
    pub right: Expression,
}

pub enum LogicalOperator {
    And,
    Or,
    Coalesce,
}

pub struct ConditionalExpression {
    pub test: Expression,
    pub consequent: Expression,
    pub alternate: Expression,
}

////////////////////////////////////////////////////////////////////////
/// 
/// Variable Expressions

pub struct VariableExpression{
    pub name: Identifier
}

pub struct AssignmentExpression {
    pub operator: AssignmentOperator,
    pub left: LValue,
    pub right: Expression,
}

pub enum LValue {
    Variable(Identifier),
    Member(MemberExpression),
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

pub struct MemberExpression {
    pub object: Expression,
    pub property: Member,
}

pub enum Member {
    Computed(Expression),
    Property(Identifier),
}

pub struct CallExpression {
    pub callee: Expression,
    pub arguments: Vec<ParameterElement>,
}

pub struct ParenthesizedExpression{
    expression: Expression
}

////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////

// SWC AST integration

use core::panic;

use swc_ecma_ast as ast;

impl From<ast::Ident> for Identifier {
    fn from(ident: ast::Ident) -> Self {
        Identifier {
            name: ident.sym.to_string(),
        }
    }
}

impl From<ast::Pat> for Pattern {
    fn from(pat: ast::Pat) -> Self {
        match pat {
            ast::Pat::Ident(ident) => Pattern::Identifier(Identifier::from(ident.id)),
            ast::Pat::Array(array) => Pattern::Array(ArrayPattern {
                elements: array.elems.into_iter().map(|e| e.map(|e| e.into()).unwrap_or(Pattern::Hole)).collect(),
            }),
            ast::Pat::Object(object) => Pattern::Object(ObjectPattern {
                properties: object
                    .props
                    .into_iter()
                    .map(|p| match p {
                        ast::ObjectPatProp::KeyValue(key_value) => {
                            PropertyPattern::KeyValue(
                                Identifier::from(key_value.key),
                                (*key_value.value).into(),
                            )
                        }
                        ast::ObjectPatProp::Assign(assign) => {
                            PropertyPattern::Optional(OptionalPattern {
                                binding: Identifier::from(assign.key),
                                default: (*assign.value.unwrap()).into(),
                            })
                        }
                        ast::ObjectPatProp::Rest(rest) => {
                            PropertyPattern::Rest((*rest.arg).into())
                        }
                    })
                    .collect(),
            }),
            _ => unimplemented!(),   
        }
    }
}

impl From<ast::Param> for Pattern {
    fn from(param: ast::Param) -> Self {
        param.pat.into()
    }
}

////////////////////////////////////////////////////////////////////////

impl From<ast::Script> for Script {
    fn from(script: ast::Script) -> Self {
        Script {
            body: script.body.into_iter().map(|stmt| stmt.into()).collect(),
        }
    }
}

impl From<ast::Stmt> for Statement {
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



impl From<ast::VarDecl> for VariableDeclaration {
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

impl From<ast::VarDeclarator> for VariableDeclarator {
    fn from(decl: ast::VarDeclarator) -> Self {
        VariableDeclarator {
            binding: decl.name.into(),
            init: decl.init.map(|init| (*init).into()),
        }
    }
}

impl From<ast::FnDecl> for FunctionDeclaration {
    fn from(decl: ast::FnDecl) -> Self {
        let mut function: FunctionExpression = decl.function.into();
        function.name = Some(decl.ident.into());
        FunctionDeclaration { function }
    }
}

impl From<ast::BlockStmt> for BlockStatement {
    fn from(stmt: ast::BlockStmt) -> Self {
        BlockStatement {
            body: stmt.stmts.into_iter().map(|stmt| stmt.into()).collect(),
        }
    }
}

impl From<ast::IfStmt> for IfStatement {
    fn from(stmt: ast::IfStmt) -> Self {
        IfStatement {
            test: (*stmt.test).into(),
            consequent: (*stmt.cons).into(),
            alternate: stmt.alt.map(|stmt| (*stmt).into()),
        }
    }
}

impl From<ast::ForStmt> for ForStatement {
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

impl From<ast::ForOfStmt> for ForOfStatement {
    fn from(stmt: ast::ForOfStmt) -> Self {
        unimplemented!()
    }
}

impl From<ast::WhileStmt> for WhileStatement {
    fn from(stmt: ast::WhileStmt) -> Self {
        WhileStatement {
            test: (*stmt.test).into(),
            body: (*stmt.body).into(),
        }
    }
}

impl From<ast::SwitchStmt> for SwitchStatement {
    fn from(stmt: ast::SwitchStmt) -> Self {
        SwitchStatement {
            discriminant: (*stmt.discriminant).into(),
            cases: stmt.cases.into_iter().map(|case| case.into()).collect(),
        }
    }
}

impl From<ast::SwitchCase> for SwitchCase {
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

impl From<ast::TryStmt> for TryStatement {
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

impl From<ast::ReturnStmt> for ReturnStatement {
    fn from(stmt: ast::ReturnStmt) -> Self {
        ReturnStatement {
            argument: stmt.arg.map(|arg| (*arg).into()),
        }
    }
}

impl From<ast::ThrowStmt> for ThrowStatement {
    fn from(stmt: ast::ThrowStmt) -> Self {
        ThrowStatement {
            argument: (*stmt.arg).into(),
        }
    }
}

impl From<ast::ExprStmt> for ExpressionStatement {
    fn from(stmt: ast::ExprStmt) -> Self {
        ExpressionStatement {
            expression: (*stmt.expr).into(),
        }
    }
}

impl From<ast::Expr> for Expression {
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

const MAX_SAFE_INTEGER: f64 = 9007199254740991.0;
const MIN_SAFE_INTEGER: f64 = -9007199254740991.0;

impl From<f64> for NumberLiteral {
    fn from(num: f64) -> Self {
        if num < MIN_SAFE_INTEGER || num > MAX_SAFE_INTEGER {
            unimplemented!("Number literal is out of range");
        } else {
            NumberLiteral{value: num.round() as i64}
        }
    }
}

impl From<ast::Lit> for Literal {
    fn from(lit: ast::Lit) -> Self {
        match lit {
            ast::Lit::Str(str) => Literal::String(StringLiteral{value: str.value.to_string()}),
            ast::Lit::Num(num) => Literal::Number(num.value.into()), // TODO: add bound checks
            ast::Lit::Bool(bool) => Literal::Boolean(BooleanLiteral{value: bool.value}),
            ast::Lit::Null(_) => Literal::Null,
            // ast::Lit::BigInt(bigint) => Literal::Bigint(bigint.value.to_string()),
            _ => unimplemented!(),
        }
    }
}

impl From<ast::ArrayLit> for ArrayExpression {
    fn from(array: ast::ArrayLit) -> Self {
        ArrayExpression {
            elements: array.elems.into_iter().map(|elem| elem.map(|elem| elem.into()).unwrap_or(ParameterElement::Parameter(Expression::Literal(Box::new(Literal::Undefined))))).collect(),
        }
    }
}

impl From<ast::ObjectLit> for ObjectExpression {
    fn from(object: ast::ObjectLit) -> Self {
        ObjectExpression {
            properties: object.props.into_iter().map(|prop| prop.into()).collect(),
        }
    }
}

impl From<ast::PropOrSpread> for ObjectElement {
    fn from(prop: ast::PropOrSpread) -> Self {
        match prop {
            ast::PropOrSpread::Prop(prop) => (*prop).into(), 
            ast::PropOrSpread::Spread(spread) => ObjectElement::Spread((*spread.expr).into())
        }
    }
}

impl From<ast::Prop> for ObjectElement {
    fn from(prop: ast::Prop) -> Self {
        match prop {
            ast::Prop::Shorthand(ident) => ObjectElement::Shorthand(ident.into()),
            ast::Prop::KeyValue(key_value) => ObjectElement::KeyValue(key_value.key.into(), (*key_value.value).into()),
            ast::Prop::Getter(getter) => ObjectElement::Getter(getter.key.into(), getter.body.unwrap().into()),
            ast::Prop::Setter(setter) => ObjectElement::Setter(setter.key.into(), setter.param.into(), setter.body.unwrap().into()),
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

impl From<ast::FnExpr> for FunctionExpression {
    fn from(fn_expr: ast::FnExpr) -> Self {
        FunctionExpression {
            name: fn_expr.ident.map(|ident| ident.into()),
            params: fn_expr.function.params.into_iter().map(|param| param.into()).collect(),
            body: fn_expr.function.body.map(|x| x.into()).unwrap_or(BlockStatement{body: vec![]}),
        }
    }
}

impl From<ast::Function> for FunctionExpression {
    fn from(function: ast::Function) -> Self {
        FunctionExpression {
            name: None,
            params: function.params.into_iter().map(|param| param.into()).collect(),
            body: function.body.map(|x| x.into()).unwrap_or(BlockStatement{body: vec![]}),
        }
    }
}

impl From<ast::ArrowExpr> for ArrowFunctionExpression {
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

impl From<ast::UnaryExpr> for UnaryExpression {
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

impl From<ast::UpdateExpr> for UpdateExpression {
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

impl From<ast::BinExpr> for BinaryExpression {
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

impl From<ast::BinExpr> for LogicalExpression {
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

impl From<ast::CondExpr> for ConditionalExpression {
    fn from(cond: ast::CondExpr) -> Self {
        ConditionalExpression {
            test: (*cond.test).into(),
            consequent: (*cond.cons).into(),
            alternate: (*cond.alt).into(),
        }
    }
}

impl From<ast::Ident> for VariableExpression {
    fn from(ident: ast::Ident) -> Self {
        VariableExpression {
            name: Identifier{name: ident.sym.to_string()},
        }
    }
}

impl From<ast::AssignExpr> for AssignmentExpression {
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

impl From<ast::PatOrExpr> for LValue {
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

impl From<ast::MemberExpr> for MemberExpression {
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

impl From<ast::CallExpr> for CallExpression {
    fn from(call: ast::CallExpr) -> Self {
        CallExpression {
            callee: call.callee.into(),
            arguments: call.args.into_iter().map(|arg| arg.into()).collect(),
        }
    }
}

impl From<ast::ExprOrSpread> for ParameterElement {
    fn from(expr_or_spread: ast::ExprOrSpread) -> Self {
        if expr_or_spread.spread.is_some() {
            ParameterElement::Spread((*expr_or_spread.expr).into())
        } else {
            ParameterElement::Parameter((*expr_or_spread.expr).into())
        }
    }
}

impl From<ast::Callee> for Expression {
    fn from(callee: ast::Callee) -> Self {
        match callee {
            ast::Callee::Expr(expr) => (*expr).into(),
            ast::Callee::Super(_) => unimplemented!(),
            ast::Callee::Import(_) => unimplemented!(),
        }
    }
}

impl From<ast::ParenExpr> for ParenthesizedExpression {
    fn from(paren: ast::ParenExpr) -> Self {
        ParenthesizedExpression {
            expression: (*paren.expr).into(),
        }
    }
}