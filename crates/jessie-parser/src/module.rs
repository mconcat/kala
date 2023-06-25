use jessie_ast::*;
use crate::parser;
use crate::{
    VecToken, Token,

    repeated_elements,
};

type ParserState = parser::ParserState<VecToken>;
type ParserError = parser::ParserError<Option<Token>>;

///////////////////////
// Module

pub fn module_body(state: &mut ParserState) -> Result<ModuleBody, ParserError> {
    let mut items = Vec::new();

    while let Some(_) = state.lookahead_1() {
        items.push(module_item(state)?);
    }

    Ok(ModuleBody(items))
}
/* 
pub fn import_declaration(state: &mut ParserState, proxy: MutableDeclarationPointer) -> Result<ImportDeclaration, ParserError> {
    unimplemented!("import declaration")
}
*/

pub fn module_item(state: &mut ParserState) -> Result<ModuleItem, ParserError> {
    match state.lookahead_1() {
        Some(Token::Const) => {
            state.proceed();
            let proxy = state.scope.declare(|proxy| {
               repeated_elements(state, None, Token::Semicolon, &|state| binding(state, &proxy.clone()), false).map(Declaration::Const) 
            })?;
            Ok(ModuleItem::ModuleDeclaration(ModuleDeclaration{
                export_clause: ExportClause::NoExport,
                declaration: proxy,
            }))
        },
        Some(Token::Let) => {
            state.err_expected("either const or function for top level declaration", Some(Token::Let))
        },
        Some(Token::Function) => {
            let proxy = state.scope.declare(|proxy| {
                function_decl(state)
            });
            Ok(ModuleItem::ModuleDeclaration(ModuleDeclaration{
                export_clause: ExportClause::NoExport,
                declaration: proxy,
            }))
        }
        Some(Token::Export) => {
            unimplemented!("export declaration")
            /* 
            state.proceed();
            let export_clause = if state.try_proceed(Token::Default) {
                ExportClause::ExportDefault
            } else {
                ExportClause::Export
            };
            let decl = module_binding(state, proxy)?;
            state.scope.settle_declaration(proxy, decl.clone());
            Ok(ModuleItem::ModuleDeclaration(ModuleDeclaration{
                export_clause,
                declaration: proxy,
            }))
            */
        },
        Some(Token::Import) => {
            unimplemented!("import declaration")
            /* 
            state.proceed();
            let decl = import_declaration(state, proxy)?;
            state.scope.settle_declaration(proxy, decl.clone());
            Ok(ModuleItem::ModuleDeclaration(ModuleDeclaration{
                export_clause: ExportClause::NoExport,
                declaration: decl,
            }))
            */
        }
        t => state.err_expected("module declaration", t),
    }
}

pub fn hardened_expr(state: &mut ParserState) -> Result<Expr, ParserError> {
    Ok(expression(state)?) // TODO
}