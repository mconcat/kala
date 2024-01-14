use std::rc::Rc;

use crate::{Declaration, Statement};

#[derive(Debug, PartialEq, Clone)]
pub struct Script<T>{
    pub used_builtins: Vec<T>,
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Module<T>{
    pub used_builtins: Vec<T>,
    pub body: Vec<ModuleItem>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ModuleItem {
    ImportDeclaration(ImportDeclaration),
    ModuleDeclaration(ModuleDeclaration),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImportDeclaration {
    import_clause: ImportClause,
    source: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImportClause {
    Namespace(String), // import * as name from source
    Named(Box<[(String, Option<String>)]>), // import { name1, name2 as name3 } from source
    Default(String), // import name from source
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExportClause {
    NoExport,
    Export,
    ExportDefault,
}

impl ExportClause {
    pub fn is_default(&self) -> bool {
        match self {
            ExportClause::ExportDefault => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ModuleDeclaration {
    pub export_clause: ExportClause,
    pub declaration: Declaration,
}