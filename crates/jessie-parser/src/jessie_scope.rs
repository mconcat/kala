use std::thread::Scope;

use jessie_ast::DeclarationKind;

use crate::{trie::Trie, parser::{ParserError, err_scope}};

#[derive(Debug, PartialEq, Clone)]
pub struct BlockScope {
    pub declarations: Trie<DeclarationKind>,
    pub uses: Trie<()>,
}

impl BlockScope {
    pub fn new() -> Self {
        Self {
            declarations: Trie::empty(),
            uses: Trie::empty(),
        }
    }

    pub fn def_variable<T>(&mut self, name: &String, decltype: DeclarationKind) -> Result<(), ParserError<T>> {
        let old_value = self.declarations.insert(name, decltype);
        if old_value.is_some() {
            err_scope(&"Variable already declared", name.clone())
        } else {
            Ok(())
        }
    }

    pub fn use_variable(&mut self, name: &String) {
        self.uses.insert(name, ());
    }

    pub fn extract(self) -> (Vec<(String, DeclarationKind)>, Vec<String>) {
        (self.declarations.iterate().into_iter().map(move |(k, v)| (k, v)).collect(), self.uses.iterate().into_iter().map(|(k, _)| k).collect())
    }

    pub fn merge_into(self, other: &mut Self) {
        let (decls, uses) = self.extract();
        // merge_into is currently called only when 
        // 1. tries to parse a cover-paren-arrowparams and
        // 2. it turns out as a parenthesis expression, merging back to parent scope
        // so it cannot have any declarations.
        if decls.len() > 0 {
            unreachable!("merge_into called with declarations");
        }
        for name in uses {
            other.use_variable(&name);
        }
    }
}