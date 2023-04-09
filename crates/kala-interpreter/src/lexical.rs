use crate::node::{self, Identifier};
use kala_ast::ast;
use hashbrown::HashSet;

#[derive(Clone, Debug)]
pub struct BlockMetadata {
    // hoisted declaration of let/const variables
    pub local_variables: Vec<Identifier>,

    // hoisted declaration/definition of local functions
    pub local_functions: Vec<Function>,

    // free variables captured from outer environment
    pub free_variables: Vec<Identifier>,
}

// using as a set
pub type EnvironmentRecord = environment_record::EnvironmentRecord<Identifier, ()>;

pub struct ScopeContext {
    pub bound_variables: EnvironmentRecord,

    pub free_variables: Vec<Identifier>,
}

pub struct Scope<'a> {
    pub ctx: &'a mut ScopeContext,
}

impl<'a> Scope<'a> {
    pub fn new(ctx: &'a mut ScopeContext) -> Self {

    }

    // https://www.haskell.org/ghc/blog/20190728-free-variable-traversals.html
    // iterates in two steps:
    // 
    pub fn block(&mut self, block: &mut node::Block) -> Vec<Identifier> {
        self.ctx.bound_variables.enter();
        self.ctx.free_variables.enter();
    }

    pub fn statement(&mut self, stmt: &mut node::Statement) {
        match &mut stmt {
            ast::Statement::VariableDeclaration(stmt) => self.variable_declaration_statement(stmt),
            ast::Statement::FunctionDeclaration(stmt) => self.function_declaration_statement(stmt),
            ast::Statement::Block(stmt) => self.block_statement(stmt),
        }
    }

    fn variable_declaration_statement(&mut self, stmt: &mut ast::VariableDeclaration) {
        for decl in &mut stmt.declarators {
            self.declare_local_variables(decl.binding.variables());
            self.declare_free_variables(decl.init)
        }
    }
}