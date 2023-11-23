use std::rc::Rc;

use jessie_ast::{*, module::{ ExportClause, Module, ModuleDeclaration, ModuleItem}};
use utils::{MapPool, FxMap, Map};

use crate::{Token,  statement::{binding, function_decl, const_decl, statement}, parser,  common::identifier, function::function_internal, JessieParserState, expression};

type ParserError = parser::ParserError<Option<Token>>;

pub fn script<T: Clone>(mut state: JessieParserState, builtins: &mut FxMap<T>) -> Result<Script<T>, ParserError> {
    let mut statements = vec![];
    while state.lookahead_1() != Some(Token::EOF) {
        statements.push(statement(&mut state)?);
    }

    // once we have fully walked through the entire script, we have to virtually 'exit' the implicit top level scope and settle the unresolved variables

    let used_builtins = state.scope.exit_module(builtins);

    Ok(Script {
        statements,
        used_builtins,
    })
}

///////////////////////
// Module

pub fn module<T: Clone>(mut state: JessieParserState, builtins: &mut FxMap<T>) -> Result<Module<T>, ParserError> {
    let mut body = vec![];

    while let Some(_) = state.lookahead_1() {
        if state.try_proceed(Token::Import) {
            unimplemented!("import declaration")
        }

        let export_clause = if state.try_proceed(Token::Export) {
            if state.try_proceed(Token::Default) {
                ExportClause::ExportDefault
            } else {
                ExportClause::Export
            }
        } else {
            ExportClause::NoExport
        };
    
        let declaration = match state.lookahead_1() {
            Some(Token::Const) => {
                const_decl(&mut state)?
            },
            Some(Token::Let) => {
                return state.err_expected("either const or function for top level declaration", Some(Token::Let))
            },
            Some(Token::Function) => {
                function_decl(&mut state)?
            } 
            t => return state.err_expected("module declaration", t),
        };

        body.push(ModuleItem::ModuleDeclaration(ModuleDeclaration {
            export_clause,
            declaration,
        }));
    }

    // once we have fully walked through the entire module, we have to virtually 'exit' the implicit top level scope and settle the unresolved variables
    // the top-level variables are settled already with the implicit module-scope(basically the whole module is in a block scope)
    // but we still need to resolve for the builtin variables, such as 'console', 'Object', etc.

    let used_builtins = state.scope.exit_module(builtins);

    Ok(Module {
        body,
        used_builtins,
    })
}
/* 
pub fn import_declaration(state: &mut ParserState, proxy: MutableDeclarationPointer) -> Result<ImportDeclaration, ParserError> {
    unimplemented!("import declaration")
}
*/
