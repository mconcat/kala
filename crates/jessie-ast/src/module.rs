use std::rc::Rc;

use utils::SharedString;

use crate::LocalDeclaration;

#[derive(Debug, PartialEq, Clone)]
pub struct ModuleBody {
    pub builtins: Vec<SharedString>, // TODO: typify
    pub globals: Vec<(ExportClause, Rc<LocalDeclaration>)>,
    // pub imports: Vec<ImportDeclaration>,
}

impl ModuleBody {
    pub fn new() -> Self {
        Self {
            builtins: Vec::new(),
            globals: Vec::new(),
            // imports: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImportDeclaration {
    import_clause: ImportClause,
    source: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImportClause {
    Namespace(String), // import * as name from source
    Named(Vec<(String, Option<String>)>), // import { name1, name2 as name3 } from source
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