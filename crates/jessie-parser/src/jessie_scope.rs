use std::{rc::Rc, cell::{Cell, RefCell}};

use jessie_ast::{DeclarationKind, Declaration, Function, Scope, MutableDeclarationPointer, DeclarationPointer, DefVariable, UseVariable};

use crate::{trie::Trie, parser::{ParserError, err_scope}};



#[derive(Debug, PartialEq, Clone)]
pub struct BlockScope {
    pub declarations: Option<Box<Vec<DeclarationPointer>>>,
    pub declaration_trie: Option<Box<Trie<MutableDeclarationPointer>>>, // Map<String, &mut Option<&Declaration>>
    pub uses: Option<Box<Trie<MutableDeclarationPointer>>>, // Map<String, &mut Option<&Declaration>>

    // as binding pattern does not recursively declare variables, we can safely store the current declaration proxy here
    pub current_declaration_proxy: MutableDeclarationPointer,
}

impl BlockScope {
    pub fn new() -> Self {
        Self {
            declarations: None,
            declaration_trie: None,
            uses: None,

            current_declaration_proxy: MutableDeclarationPointer::new(),
        }
    }

    // checks if the variable is already declared in this scope
    pub fn def_variable<T>(&mut self, name: &String) -> Result<DefVariable, ParserError<T>> {
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
        Ok(DefVariable { name: name.clone() })
    }

    // should be called after all the patterns have passed def_variable
    pub fn settle_declaration(&mut self, decl: DeclarationPointer) {
        println!("settle_declaration: {:?} {:?}", self, decl);
        self.current_declaration_proxy.settle(decl.clone());
        if let Some(decls) = &mut self.declarations {
            decls.push(decl);
        } else {
            self.declarations = Some(Box::new(vec![decl]))
        }
        self.current_declaration_proxy = MutableDeclarationPointer::new();
    }

    pub fn use_variable(&mut self, name: &String) -> UseVariable {
        let decl = if let Some(uses) = &mut self.uses {
            let ptr = uses.get_mut(name);
            if let Some(ptr) = ptr {
                ptr.clone()
            } else {
                let decl = MutableDeclarationPointer::new();
                uses.insert(name, decl.clone());
                decl
            }
        } else {
            let mut uses = Trie::empty();
            let decl = MutableDeclarationPointer::new();
            uses.insert(name, decl.clone());
            self.uses = Some(Box::new(uses));
            decl
        };

        UseVariable { name: name.clone(), decl }
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

    pub fn take(self) -> (Option<Box<Vec<DeclarationPointer>>>/*, Vec<(String, MutableDeclarationPointer)>*/, Vec<(String, MutableDeclarationPointer)>) {
        let Self{declarations, declaration_trie, uses, current_declaration_proxy} = self;

        // let mut bound_uses: Vec<(String, MutableDeclarationPointer)> = vec![];
        let mut unbound_uses: Vec<(String, MutableDeclarationPointer)> = vec![];
        // iterate through all the used variables and check if they are declared locally - bind them
        if let Some(uses) = uses {
            if let Some(mut declaration_trie) = declaration_trie {
                for (name, ptr) in &mut uses.iterate() {
                    if let Some(decl) = declaration_trie.get(&name) {
                        ptr.settle(decl.get().unwrap());
                    } else {
                        unbound_uses.push((name.clone(), ptr.clone()));
                    }
                }
            } else {
                for (name, ptr) in uses.iterate() {
                    unbound_uses.push((name.clone(), ptr));
                }
            }
        };
        
        (declarations,/*bound_uses */ unbound_uses)
    }

    pub fn assert_equivalence(&mut self, name: &String, mut var: MutableDeclarationPointer) {
        if self.declaration_trie.is_none() {
            self.declaration_trie = Some(Box::new(Trie::empty()));
        }

        let mut declaration_trie = self.declaration_trie.as_mut().unwrap();
        let parent_var = declaration_trie.get(name);
        if parent_var.is_none() {
            declaration_trie.insert(name, var); // TODO: optimize redundent trie traverse
            return
        }

        let parent_var = parent_var.unwrap();
        var.replace(parent_var);    
    }
}