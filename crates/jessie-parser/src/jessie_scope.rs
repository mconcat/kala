use std::{rc::Rc, cell::{Cell, RefCell}};

use jessie_ast::{DeclarationKind, Declaration, Function, Scope};

use crate::{trie::Trie, parser::{ParserError, err_scope}};

#[derive(Debug, PartialEq, Clone)]
pub struct BlockScope {
    pub declarations: Option<Box<Vec<Rc<Declaration>>>>,
    pub declaration_trie: Option<Box<Trie<Rc<RefCell<Option<Rc<Declaration>>>>>>>, // Map<String, Declaration>
    pub uses: Option<Box<Trie<()>>>, // Set<String>

    // as binding pattern does not recursively declare variables, we can safely store the current declaration proxy here
    pub current_declaration_proxy: Rc<RefCell<Option<Rc<Declaration>>>>,
}

impl BlockScope {
    pub fn new() -> Self {
        Self {
            declarations: None,
            declaration_trie: None,
            uses: None,

            current_declaration_proxy: Rc::new(RefCell::new(None)),
        }
    }

    // checks if the variable is already declared in this scope
    pub fn def_variable<T>(&mut self, name: &String) -> Result<(), ParserError<T>> {
        println!("def_variable: {}", name);
        if let Some(declaration_tree) = &mut self.declaration_trie {
            if declaration_tree.insert(name, self.current_declaration_proxy.clone()).is_some() {
                return err_scope("variable is already declared in this scope", name.clone());
            }
        } else {
            let mut declaration_tree = Trie::empty();
            declaration_tree.insert(name, self.current_declaration_proxy.clone());
            self.declaration_trie = Some(Box::new(declaration_tree));
        }
        Ok(())
    }

    // should be called after all the patterns have passed def_variable
    pub fn settle_declaration(&mut self, decl: Rc<Declaration>) {
        println!("settle_declaration: {:?} {:?}", self, decl);
        *self.current_declaration_proxy.borrow_mut() = Some(decl.clone());
        if let Some(decls) = &mut self.declarations {
            decls.push(decl);
        } else {
            self.declarations = Some(Box::new(vec![decl]))
        }
        self.current_declaration_proxy = Rc::new(RefCell::new(None));
    }

    pub fn use_variable(&mut self, name: &String) {
        if let Some(uses) = &mut self.uses {
            uses.insert(name, ());
        } else {
            let mut uses = Trie::empty();
            uses.insert(name, ());
            self.uses = Some(Box::new(uses));
        }
    }

    pub fn merge_into(self, other: &mut Self) {
        let Self{declarations, declaration_trie, uses, current_declaration_proxy} = self;
        // merge_into is currently called only when 
        // 1. tries to parse a cover-paren-arrowparams and
        // 2. it turns out as a parenthesis expression, merging back to parent scope
        // so it cannot have any declarations.
        // TODO: what happens if a parenthesized expression includes a named function expression?
        if declarations.is_some_and(|decls| !decls.is_empty()) {
            unreachable!("merge_into called with declarations");
        }
        if let Some(uses) = uses {
            for (name, _) in uses.iterate() {
                other.use_variable(&name);
            }
        }
    }

    pub fn take(self) -> Scope {
        let Self{declarations, declaration_trie, uses, current_declaration_proxy} = self;

        let mut uses_result: Vec<(String, Option<Rc<Declaration>>)> = Vec::new();
        // iterate through all the used variables and check if they are declared locally - bind them
        if let Some(uses) = uses {
            if let Some(mut declaration_trie) = declaration_trie {
                for (name, _) in uses.iterate() {
                    if let Some(decl) = declaration_trie.get(&name) {
                        uses_result.push((name.clone(), decl.borrow().clone()));
                    } else {
                        uses_result.push((name.clone(), None));
                    }
                }
            } else {
                for (name, _) in uses.iterate() {
                    uses_result.push((name.clone(), None));
                }
            }
        };
        
        Scope{declarations, uses: uses_result}
    }
}