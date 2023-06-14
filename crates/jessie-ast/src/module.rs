#[derive(Debug, PartialEq, Clone)]
pub struct ModuleBody(pub Vec<ModuleItem>);

#[derive(Debug, PartialEq, Clone)]
pub enum ModuleItem {
    // ImportDeclaration(ImportDeclaration),
    ModuleDeclaration(ModuleDeclaration),
}
/* 
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
*/
#[derive(Debug, PartialEq, Clone)]
pub enum ExportClause {
    NoExport,
    Export,
    ExportDefault,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ModuleDeclaration {
    pub export_clause: ExportClause,
    // Using MutableDeclarationPointer as top level const/functions might be used before their declaration
    pub declaration: MutableDeclarationPointer, // Must be pointing either Function or Const, TODO: enforce
}