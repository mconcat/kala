use std::rc::Rc;

use jessie_ast::{*, module::{ModuleBody, ExportClause}};
use utils::{MapPool, FxMap, Map};

use crate::{Token, jessie_parser::repeated_elements, statement::{binding}, parser, expression, common::identifier, function::function_internal, JessieParserState, scope::LexicalScope};

type ParserError = parser::ParserError<Option<Token>>;

///////////////////////
// Module


pub fn module(mut state: JessieParserState, mut builtins: FxMap<Option<u32>>) -> Result<ModuleBody, ParserError> {
    let mut modulebody = ModuleBody::new();

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
    
        match state.lookahead_1() {
            Some(Token::Const) => {
                state.proceed();
                let (pattern, init) = binding(&mut state)?;
                let (index, declaration) = state.scope.declare_const(pattern, init.unwrap()).ok_or(ParserError::DuplicateDeclaration)?;
                modulebody.globals.push((export_clause, declaration))
            },
            Some(Token::Let) => {
                return state.err_expected("either const or function for top level declaration", Some(Token::Let))
            },
            Some(Token::Function) => {
                state.proceed();
                let name = identifier(&mut state)?;
                let parent_scope = state.enter_block_scope();
                // TODO: support recursive reference to function
                let function = function_internal(&mut state, FunctionName::Named(name))?;
                state.exit_block_scope(parent_scope);
                let (index, declaration) = state.scope.declare_function(function).ok_or(ParserError::DuplicateDeclaration)?;
                modulebody.globals.push((export_clause, declaration))
            } 
            t => return state.err_expected("module declaration", t),
        }
    }

    // once we have fully walked through the entire module, we have to virtually 'exit' the implicit top level scope and settle the unresolved variables
    // the top-level variables are settled already with the implicit module-scope(basically the whole module is in a block scope)
    // but we still need to resolve for the builtin variables, such as 'console', 'Object', etc.

    // ignore declarations, we have manually handled them in the above loop
    let LexicalScope{declarations: _, variables} = state.scope;

    let mut ptrs = state.map_pool.drain(variables);

    for (name, mut ptr) in ptrs {
        if ptr.is_uninitialized() {
            // not declared anywhere, probably builtin

            let builtin_index = builtins.get(name.clone()).ok_or(ParserError::UnresolvedVariable(name.clone()))?;

            if let Some(builtin_index) = builtin_index {
                ptr.set(DeclarationIndex::Builtin(*builtin_index), vec![]).unwrap();
            } else {
                let new_builtin_index = modulebody.builtins.len() as u32;
                builtin_index.insert(new_builtin_index);
                modulebody.builtins.push(name.clone());
                ptr.set(DeclarationIndex::Builtin(new_builtin_index), vec![]).unwrap();
            }
        }
    }

    Ok(modulebody)
}
/* 
pub fn import_declaration(state: &mut ParserState, proxy: MutableDeclarationPointer) -> Result<ImportDeclaration, ParserError> {
    unimplemented!("import declaration")
}
*/
