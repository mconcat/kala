use std::{rc::Rc, mem, cell::{Cell, OnceCell, RefCell}, fmt::Debug};

use static_assertions::{assert_eq_size, assert_eq_align};

use crate::{UnaryOp, BinaryOp};


#[repr(u8)]
pub enum StatementDiscriminant {
    LocalDeclaration = 0,
    Block = 2,
    IfStatement = 3,
    // ForStatement = 4
    WhileStatement = 5,
    Continue = 6,
    Break = 7,
    Return = 8,
    ReturnEmpty = 9,
    Throw = 10,
    // TryStatement = 11,
    ExprStatement = 12,
}

#[repr(u8)]
// StatementItem in Jessie
#[derive(PartialEq, Clone)]
pub enum Statement {
    // The actual declaration is stored in the innermost function. DeclarationIndicies point to them.
    // When encountered, declaration statements initializes the variable to undefined, or with init value.
    // TODO: we can actually remove the declaration indicies as they always match with the order they appears inside the function, but I will just use u32 indices for now - refactor later
    LocalDeclaration(Box<Declaration>) = StatementDiscriminant::LocalDeclaration as u8,
    Block(Box<Block>) = StatementDiscriminant::Block as u8,
    IfStatement(Box<IfStatement>) = StatementDiscriminant::IfStatement as u8,
    // ForStatement(ForStatement),
    WhileStatement(Box<WhileStatement>) = StatementDiscriminant::WhileStatement as u8,
    Continue = StatementDiscriminant::Continue as u8,
    Break = StatementDiscriminant::Break as u8,
    Return(Box<Expr>) = StatementDiscriminant::Return as u8,
    ReturnEmpty = StatementDiscriminant::ReturnEmpty as u8,
    Throw(Box<Expr>) = StatementDiscriminant::Throw as u8,
    // TryStatement(TryStatement),
    ExprStatement(Box<Expr>) = StatementDiscriminant::ExprStatement as u8,
}

#[derive(PartialEq, Clone)]
pub struct Block {
    pub declarations: Box<[Declaration]>,

    pub statements: Box<[Statement]>,
}

impl Block {
    pub fn new(declarations: Box<[Declaration]>, statements: Box<[Statement]>) -> Self {
        Block{declarations, statements}
    }
}

#[derive(PartialEq, Clone)]
pub struct IfStatement {
    pub condition: Expr,
    pub consequent: Block,
    pub alternate: ElseArm,
}


#[derive(PartialEq, Clone)]
pub enum ElseArm {
    NoElse,
    Else(Block),
    ElseIf(Box<IfStatement>),
}

#[derive(PartialEq, Clone)]
pub struct WhileStatement {
    pub condition: Expr,
    pub body: Block,
}

#[repr(u8)]
pub enum ExprDiscriminant {
    DataLiteral = 0,
    Array = 1,
    Record = 2,
    Function = 3,
    Assignment = 5,
    CondExpr = 6,
    BinaryExpr = 7,
    UnaryExpr = 8,
    CallExpr = 9,
    // QuasiExpr() = 10
    ParenedExpr = 11,
    Variable = 12,
    Spread = 13,
}

#[derive(PartialEq, Clone)]
#[repr(u8)]
pub enum Expr {
    DataLiteral(Box<DataLiteral>) = ExprDiscriminant::DataLiteral as u8,
    Array(Box<Array>) = ExprDiscriminant::Array as u8,
    Record(Box<Record>) = ExprDiscriminant::Record as u8,
    Function(Box<Function>) = ExprDiscriminant::Function as u8,
    Assignment(Box<Assignment>) = ExprDiscriminant::Assignment as u8,
    CondExpr(Box<CondExpr>) = ExprDiscriminant::CondExpr as u8,
    BinaryExpr(Box<BinaryExpr>) = ExprDiscriminant::BinaryExpr as u8,
    UnaryExpr(Box<UnaryExpr>) = ExprDiscriminant::UnaryExpr as u8,
    CallExpr(Box<CallExpr>) = ExprDiscriminant::CallExpr as u8,
    // QuasiExpr() = 10
    ParenedExpr(Box<Expr>) = ExprDiscriminant::ParenedExpr as u8,
    Variable(Box<Variable>) = ExprDiscriminant::Variable as u8,
    Spread(Box<Expr>) = ExprDiscriminant::Spread as u8, // for array elements
}

#[repr(transparent)]
#[derive(PartialEq, Clone)]
pub struct Array(pub Box<[Expr]>);

#[derive(PartialEq, Clone)]
pub struct KeyValue {
    pub key: Field,
    pub value: Expr,
}


#[repr(transparent)]
#[derive(Debug, PartialEq, Clone)]
pub struct Record(pub Box<[PropDef]>);

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum PropDef {
    KeyValue(Box<Field>, Expr),
    Shorthand(Box<Field>, Box<Variable>),
    Spread(Expr),
    Getter(Box<Function>),
    Setter(Box<Function>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub name: Rc<str>,
}

#[derive(PartialEq, Clone)]
pub enum DataLiteral {
    Null,
    False,
    True,
    Integer(i64),
    Decimal(i64, u64),
    String(Rc<str>),
    Undefined,
    Bigint(bool, Box<[u64]>),
}



#[derive(PartialEq, Clone)]
pub struct BinaryExpr(pub BinaryOp, pub Expr, pub Expr);

#[derive(PartialEq, Clone)]
pub struct CondExpr(pub Expr, pub Expr, pub Expr);

#[derive(PartialEq, Clone)]
pub struct UnaryExpr {
    pub op: Box<[UnaryOp]>,
    pub expr: Expr,
}

#[repr(u8)]
#[derive(PartialEq, Clone)]
pub enum CallPostOp {
    Index(Expr) = 0,
    Member(Rc<str>) = 1,
    // QuasiExpr = 2
    Call(Box<[Expr]>) = 3,
}


#[derive(PartialEq, Clone)]
pub struct CallExpr {
    pub expr: Expr,
    pub post_ops: Box<[CallPostOp]>,
}
#[repr(C)]
#[derive(PartialEq, Clone)]
pub struct Assignment(pub AssignOp, pub LValue, pub Expr);


#[repr(u8)]
#[derive(PartialEq, Clone)]
pub enum AssignOp {
    Assign = 0,
    AssignAdd,
    AssignSub,
    AssignMul,
    AssignDiv,
    AssignMod,
    AssignExp,
    AssignLShift,
    AssignRShift,
    AssignURShift,
    AssignBitAnd,
    AssignBitXor,
    AssignBitOr,
}


#[repr(u8)]
#[derive(PartialEq, Clone)]
pub enum LValue {
    CallLValue(Box<CallLValue>) = ExprDiscriminant::CallExpr as u8,
    Variable(Box<Variable>) = ExprDiscriminant::Variable as u8,
}

#[repr(u8)]
#[derive(PartialEq, Clone)]
pub enum LValueCallPostOp {
    Index(Expr) = 0,
    Member(Rc<str>) = 1,
}

#[derive(PartialEq, Clone)]
pub struct CallLValue {
    pub expr: Expr,
    pub post_ops: Box<[LValueCallPostOp]>,
}


assert_eq_size!(LValue, Expr);
assert_eq_size!(LValueCallPostOp, CallPostOp);
assert_eq_align!(LValue, Expr);
assert_eq_align!(LValueCallPostOp, CallPostOp);

impl From<LValue> for Expr {
    fn from(lv: LValue) -> Self {
        // Super unsafe, add bunch of test cases later
        // we are "widening" the enum, so it's relatively safe
        unsafe { mem::transmute(lv) }
    }
}

impl From<Expr> for LValue {
    fn from(value: Expr) -> Self {

         // must be called only when the expr is transmutable to LValue
         // Super super unsafe
         // we are "narrowing" the enum, so it's unsafe
         // maybe we should not use From trait for this
        unsafe { mem::transmute(value) }
    }
}

assert_eq_size!(Pattern, Expr);
assert_eq_align!(Pattern, Expr);

// Pattern is a subset of Expr
#[repr(u8)]
#[derive(PartialEq, Clone)]
pub enum Pattern {
    Rest(Box<Pattern>) = ExprDiscriminant::Spread as u8,
    Optional(Box<OptionalPattern>) = ExprDiscriminant::Assignment as u8,
    ArrayPattern(Box<ArrayPattern>) = ExprDiscriminant::Array as u8, // only Vec<Param> form is valid
    RecordPattern(Box<RecordPattern>) = ExprDiscriminant::Record as u8,
    Variable(Box<Variable>) = ExprDiscriminant::Variable as u8,
}

impl Pattern {
    pub fn optional(variable: Variable, expr: Expr) -> Self {
        Pattern::Optional(Box::new(OptionalPattern(OptionalOp::Optional, LValueOptional::Variable(Box::new(variable)), expr)))
    }
}

#[repr(C)]
#[derive(PartialEq, Clone)]
pub struct OptionalPattern(pub OptionalOp, pub LValueOptional, pub Expr);

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum OptionalOp {
    Optional = AssignOp::Assign as u8,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum LValueOptional {
    Variable(Box<Variable>) = 12, // LValue::Variable
}

// ArrayPattern is a subset of Expr::Array
#[repr(transparent)]
#[derive(PartialEq, Clone)]
pub struct ArrayPattern(pub Box<[Pattern]>);


// RecordPattern is a subset of Expr::Record
#[repr(transparent)]
#[derive(PartialEq, Clone)]
pub struct RecordPattern(pub Box<[PropParam]>);

#[repr(u8)]
#[derive(PartialEq, Clone)]
pub enum PropParam {
    KeyValue(Box<Field>, Pattern),
    Shorthand(Box<Field>, Box<Variable>),
    Rest(Box<Variable>),
}

#[derive(PartialEq, Clone)]
pub enum Declaration {
    Const(Box<[VariableDeclaration]>),
    Let(Box<[VariableDeclaration]>),
    Function(Rc<RefCell<Function>>),
}

#[derive(PartialEq, Clone)]
pub struct VariableDeclaration {
    pub pattern: Pattern,
    pub value: Option<Expr>,
}

#[derive(PartialEq, Clone)]
pub enum FunctionName {
    Arrow,
    Anonymous,
    Named(Rc<str>),
}

#[derive(PartialEq, Clone)]
pub enum ExprOrBlock {
    Expr(Expr),
    Block(Block),
}

#[derive(PartialEq, Clone)]
pub struct Function {
    pub name: FunctionName, 

    pub parameters: Box<[Pattern]>,

    pub body: ExprOrBlock,

    pub scope: Option<Box<FunctionScope>>,
}

impl Function {
    pub fn get_name(&self) -> Option<Rc<str>> {
        match self.name {
            FunctionName::Arrow => None,
            FunctionName::Anonymous => None,
            FunctionName::Named(ref name) => Some(name.clone()),
        }
    }

    pub fn captures(&self) -> &[Variable] {
        match &self.scope {
            Some(scope) => &scope.captures,
            None => panic!("Function::captures() called on function without scope"),
        }
    }

    pub fn locals(&self) -> &[LocalVariable] {
        match &self.scope {
            Some(scope) => &scope.locals,
            None => panic!("Function::locals() called on function without scope"),
        }
    }

    pub fn functions(&self) -> &[(Variable, Rc<RefCell<Function>>)] {
        match &self.scope {
            Some(scope) => &scope.functions,
            None => panic!("Function::functions() called on function without scope"),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct LocalVariable {
    pub var: Variable,
    pub is_escaping: bool,
}

impl LocalVariable {
    pub fn new(var: Variable) -> Self {
        LocalVariable {
            var,
            is_escaping: false,
        }
    }

    pub fn escaping(var: Variable) -> Self {
        LocalVariable {
            var,
            is_escaping: true,
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct FunctionScope {
    pub parameters: Box<[LocalVariable]>,
    pub captures: Box<[Variable]>, // evaluated in parent context
    pub locals: Box<[LocalVariable]>, // evaluate in current context
    pub functions: Box<[(Variable, Rc<RefCell<Function>>)]>, // list of functions declared in this scope
}

impl FunctionScope {
    pub fn new(parameters: &[LocalVariable], captures: &[Variable], locals: &[LocalVariable], functions: &[(Variable, Rc<RefCell<Function>>)]) -> Self {
        FunctionScope {
            parameters: parameters.into(),
            captures: captures.into(),
            locals: locals.into(),
            functions: functions.into(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum VariableIndex {
    Captured(u32),
    Local(/*is_const*/bool, u32),
    Parameter(u32),
    Static(u32),
}

impl VariableIndex {
    pub fn unwrap_local(&self) -> u32 {
        match self {
            Self::Local(_, index) => *index,
            _ => panic!("unwrap_local"),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Variable {
    pub name: Rc<str>,
    pub pointer: Rc<OnceCell<VariableIndex>>,
}

impl Variable {
    pub fn declared(name: Rc<str>, pointer: VariableIndex) -> Self {
        Variable {
            name,
            pointer: Rc::new(OnceCell::from(pointer)),
        }
    }

    pub fn new(name: Rc<str>) -> Self {
        Variable::hoisted(name)
    }

    pub fn hoisted(name: Rc<str>) -> Self {
        Variable {
            name,
            pointer: Rc::new(OnceCell::new()),
        }
    }

    pub fn is_declared(&self) -> bool {
        return self.pointer.get().is_some();
    }
    
    pub fn index(&self) -> VariableIndex {
        self.pointer.get().unwrap().clone()
    }

    pub fn index_local(&self) -> u32 {
        match self.index() {
            VariableIndex::Local(is_const, index) => index,
            _ => panic!("variable index not local: {:?}", self),
        }
    }
}














impl Debug for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::LocalDeclaration(decl) => write!(f, "{:?}", decl),
            Statement::Block(block) => write!(f, "{:?}", block),
            Statement::IfStatement(if_statement) => write!(f, "{:?}", if_statement),
            Statement::WhileStatement(while_statement) => write!(f, "{:?}", while_statement),
            Statement::Continue => write!(f, "continue"),
            Statement::Break => write!(f, "break"),
            Statement::Return(expr) => write!(f, "return {:?}", expr),
            Statement::ReturnEmpty => write!(f, "return"),
            Statement::Throw(expr) => write!(f, "throw {:?}", expr),
            Statement::ExprStatement(expr) => write!(f, "{:?}", expr),
        }?;
        write!(f, ";")
    }
}


impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.statements.iter();
        if let Some(first) = iter.next() {
            write!(f, "{{{:?}", first)?;
            for statement in iter {
                write!(f, "{:?}", statement)?;
            }
            write!(f, "}}")
        } else {
            write!(f, "{{}}")
        }
    }
}

impl Debug for IfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "if ({:?}) {:?} {:?}", self.condition, self.consequent, self.alternate)
    }
}


impl Debug for ElseArm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElseArm::NoElse => Ok(()),
            ElseArm::Else(block) => write!(f, "else {:?}", block),
            ElseArm::ElseIf(if_statement) => write!(f, "else {:?}", if_statement),
        }
    }
}

impl Debug for WhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "while ({:?}) {:?}", self.condition, self.body)
    }
}



impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::DataLiteral(data) => write!(f, "{:?}", data),
            Expr::Array(array) => write!(f, "{:?}", array),
            Expr::Record(record) => write!(f, "{:?}", record),
            Expr::Function(func) => write!(f, "{:?}", func),
            Expr::Assignment(assignment) => write!(f, "{:?}", assignment),
            Expr::CondExpr(cond_expr) => write!(f, "{:?}", cond_expr),
            Expr::BinaryExpr(binary_expr) => write!(f, "{:?}", binary_expr),
            Expr::UnaryExpr(unary_expr) => write!(f, "{:?}", unary_expr),
            Expr::CallExpr(call_expr) => write!(f, "{:?}", call_expr),
            Expr::ParenedExpr(parened_expr) => write!(f, "({:?})", parened_expr),
            Expr::Variable(variable) => write!(f, "{:?}", variable),
            Expr::Spread(expr) => write!(f, "{:?}...", expr),
        }
    }
}

impl Debug for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.iter();
        write!(f, "[")?;
        if let Some(first) = iter.next() {
            write!(f, "{:?}", first)?;
            for item in iter {
                write!(f, ", {:?}", item)?;
            }
        }
        write!(f, "]")
    }
}


impl Debug for KeyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}", self.key, self.value)
    }
}



impl Debug for DataLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataLiteral::Null => write!(f, "null"),
            DataLiteral::False => write!(f, "false"),
            DataLiteral::True => write!(f, "true"),
            DataLiteral::Integer(int) => write!(f, "{}", int),
            DataLiteral::Decimal(int, frac) => write!(f, "{}.{}", int, frac), // TODO
            DataLiteral::String(string) => write!(f, "\"{:?}\"", string),
            DataLiteral::Undefined => write!(f, "undefined"),
            DataLiteral::Bigint(sign, digits) => {
                if *sign {
                    write!(f, "-")?;
                }
                for digit in digits.iter() {
                    write!(f, "{:x}", digit)?;
                }
                write!(f, "n")?;
                Ok(())
            }
        }
    }
}


impl Debug for BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?} {:?} {:?})", self.1, self.0, self.2)
    }
}

impl Debug for CondExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ? {:?} : {:?}", self.0, self.1, self.2)
    }
}


impl Debug for UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for op in self.op.iter() {
            write!(f, "{:?}", op)?;
        }
        write!(f, "{:?}", self.expr)
    }
}

impl Debug for CallPostOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CallPostOp::Index(expr) => write!(f, "[{:?}]", expr),
            CallPostOp::Member(member) => write!(f, ".{:?}", member),
            CallPostOp::Call(args) => {
                write!(f, "(")?;
                let mut iter = args.iter();
                if let Some(first) = iter.next() {
                    write!(f, "{:?}", first)?;
                    for arg in iter {
                        write!(f, ", {:?}", arg)?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}

impl Debug for CallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.expr)?;
        for op in self.post_ops.iter() {
            write!(f, "{:?}", op)?;
        }
        Ok(())
    }
}

impl Debug for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Assignment(op, lv, expr) = self;
        write!(f, "({:?} {:?} {:?})", lv, op, expr)
    }
}

impl Debug for AssignOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            AssignOp::Assign => "=",
            AssignOp::AssignAdd => "+=",
            AssignOp::AssignSub => "-=",
            AssignOp::AssignMul => "*=",
            AssignOp::AssignDiv => "/=",
            AssignOp::AssignMod => "%=",
            AssignOp::AssignExp => "**=",
            AssignOp::AssignLShift => "<<=",
            AssignOp::AssignRShift => ">>=",
            AssignOp::AssignURShift => ">>>=",
            AssignOp::AssignBitAnd => "&=",
            AssignOp::AssignBitXor => "^=",
            AssignOp::AssignBitOr => "|=",
        };
        write!(f, "{}", op)
    }
}

impl Debug for LValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LValue::CallLValue(call) => write!(f, "{:?}", call),
            LValue::Variable(var) => write!(f, "{:?}", var),
        }
    }

}
impl Debug for LValueCallPostOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LValueCallPostOp::Index(expr) => write!(f, "[{:?}]", expr),
            LValueCallPostOp::Member(name) => write!(f, ".{:?}", name),
        }
    }

}
impl Debug for CallLValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let CallLValue { expr, post_ops } = self;
        write!(f, "{:?}", expr)?;
        for op in post_ops.iter() {
            write!(f, "{:?}", op)?
            ;
        }
        Ok(())
    }
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pattern::Rest(pattern) => write!(f, "{:?}...", pattern),
            Pattern::Optional(pattern) => write!(f, "{:?}", pattern),
            Pattern::ArrayPattern(pattern) => write!(f, "{:?}", pattern),
            Pattern::RecordPattern(pattern) => write!(f, "{:?}", pattern),
            Pattern::Variable(variable) => write!(f, "{:?}", variable),
        }
    }
}

impl Debug for OptionalPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.1 {
            LValueOptional::Variable(variable) => write!(f, "{:?} = {:?}", variable, self.2),
        }
    }
}



impl Debug for ArrayPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.iter();
        write!(f, "[")?;
        if let Some(first) = iter.next() {
            write!(f, "{:?}", first)?;
            for item in iter {
                write!(f, ", {:?}", item)?;
            }
        }
        write!(f, "]")
    }
}

impl Debug for RecordPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.iter();
        write!(f, "{{")?;
        if let Some(first) = iter.next() {
            write!(f, "{:?}", first)?;
            for item in iter {
                write!(f, ", {:?}", item)?;
            }
        }
        write!(f, "}}")
    }
}

impl Debug for PropParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PropParam::KeyValue(field, pattern) => write!(f, "{:?}: {:?}", field, pattern),
            PropParam::Shorthand(field, variable) => write!(f, "{:?}", variable),
            PropParam::Rest(variable) => write!(f, "...{:?}", variable),
        }
    }
}


impl Debug for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Declaration::Const(decls) => {
                let mut iter = decls.iter();
                let mut result = String::new();
                if let Some(first) = iter.next() {
                    result.push_str(&format!("{:?}", first));
                    for decl in iter {
                        result.push_str(&format!(", {:?}", decl));
                    }
                }
                write!(f, "const {}", result)
            },
            Declaration::Let(decls) => {
                let mut iter = decls.iter();
                let mut result = String::new();
                if let Some(first) = iter.next() {
                    result.push_str(&format!("{:?}", first));
                    for decl in iter {
                        result.push_str(&format!(", {:?}", decl));
                    }
                }
                write!(f, "let {}", result)
            },
            Declaration::Function(func) => write!(f, "{:?}", func.borrow()),
        }
    }

}


impl Debug for VariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Some(value) => write!(f, "{:?} = {:?}", self.pattern, value),
            None => write!(f, "{:?}", self.pattern),
        }
    }
}

impl Debug for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(ptr) = self.pointer.get() {
            write!(f, "@{:?}", ptr)?;
        }
        Ok(())
    }
}

impl Debug for FunctionScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.captures.len() > 0 {
            write!(f, "[")?;
            for capture in self.captures.iter() {
                write!(f, "{:?}", capture)?;
            }
            write!(f, "]")?;
        }

        if self.locals.len() > 0 {
            write!(f, "{{")?;
            for capture in self.locals.iter() {
                write!(f, "{:?}", capture)?;
            }
            write!(f, "}}")?;
        }

        if self.functions.len() > 0 {
            write!(f, "{{")?;
            for (variable, function) in self.functions.iter() {
                write!(f, "{:?}", variable)?;
            }
            write!(f, "}}")?;
        }

        Ok(())
    }
}


fn render_parameters(parameters: &Box<[Pattern]>) -> String {
    let mut iter = parameters.iter();
    let mut result = String::new();
    if let Some(first) = iter.next() {
        result.push_str(&format!("{:?}", first));
        for parameter in iter {
            result.push_str(&format!(", {:?}", parameter));
        }
    }
    result
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            FunctionName::Arrow => write!(f, "({}) =>", render_parameters(&self.parameters)),
            FunctionName::Anonymous => write!(f, "function({})", render_parameters(&self.parameters)),
            FunctionName::Named(name) => write!(f, "function {}({})", name, render_parameters(&self.parameters)),
        }?;

        if let Some(scope) = self.scope.clone() {
            write!(f, "{:?}", scope)?;
        }

        match self.body {
            ExprOrBlock::Block(ref block) => write!(f, " {:?}", block),
            ExprOrBlock::Expr(ref expr) => write!(f, " {:?}", expr),
        }
    }
}

impl Debug for LocalVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}", self.var, if self.is_escaping { "!" } else { "" })
    }
}

impl<const n: usize> From<&[Statement; n]> for Block {
    fn from(value: &[Statement; n]) -> Self {
        Block {
            statements: Box::from(value.as_slice()),
            declarations: value.iter().filter_map(|stmt| match stmt {
                Statement::LocalDeclaration(decl) => Some(*decl.clone()),
                _ => None,
            }).collect::<Vec<Declaration>>().into_boxed_slice(),
        }
    }
}

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        Pattern::Variable(Box::new(Variable::new(Rc::from(value))))
    }
}

impl From<usize> for Expr {
    fn from(value: usize) -> Self {
        Expr::DataLiteral(Box::new(DataLiteral::Integer(value as i64)))
    }
}

impl From<Variable> for Pattern {
    fn from(value: Variable) -> Self {
        Pattern::Variable(Box::new(value))
    }
}

impl From<Variable> for Expr {
    fn from(value: Variable) -> Self {
        Expr::Variable(Box::new(value))
    }
}