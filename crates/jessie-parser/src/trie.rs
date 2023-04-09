// just a dead simple trie.
use std::{str::Chars, fmt::Debug, borrow::BorrowMut};

#[derive(Debug, PartialEq, Clone)]
pub struct Trie<V: Sized+Clone+Debug+PartialEq> {
    pub root: TrieNode<V>,
}

impl<V: Sized+Clone+Debug+PartialEq> Trie<V> {
    pub fn empty() -> Self {
        Self {
            root: TrieNode {
                extension: "".to_string(),
                value: None,
                branch: vec![],
            }
        }
    }

    pub fn insert(&mut self, key: &String, value: V) {
        self.root.insert(&mut key.chars(), value);
    }

    pub fn get(&mut self, key: &String) -> Option<V> {
        self.root.reference(&mut key.chars()).map(|v| v.clone())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TrieNode<V: Sized+Clone+Debug+PartialEq> {
    pub extension: String,
    pub value: Option<V>,
    pub branch: Vec<(char, Box<TrieNode<V>>)>, // sorted
}

impl<V: Sized+Clone+Debug+PartialEq> TrieNode<V> {
    fn reference<'a>(&mut self, key: &mut Chars<'a>) -> Option<&mut V> {
        for c in self.extension.chars() {
            match key.next() {
                Some(seek) if seek == c => {},
                _ => return None
            }
        }

        match key.next() {
            Some(seek) => self.branch.binary_search_by_key(&seek, |(k, _)| *k).ok().and_then(|i| self.branch.get_mut(i)).and_then(|(_, child)| child.reference(key)),
            None => self.value.as_mut().map(|v| v.borrow_mut()),
        }
    }

    fn make_branch(&mut self, diverge_point: usize, branch_key: &str, branch_value: V) {
        // { extension ===(value)===> [branch] } to
        // { prefix ===(None)===> [(divergence_char, { suffix ===(value)===>[branch]] }), (branch_divergence_char, { branch_suffix ===(new_value)===>[] })] }

        let (prefix, divergence) = self.extension.split_at(diverge_point);
        let (divergence_char, suffix) = divergence.split_at(1);

        // new key is a prefix of the existing extension
        println!("{:?}, {:?}, {:?}, {:?}, {:?}", diverge_point, branch_key, branch_value, self.extension, prefix);
        if branch_key.len() == 0 {
            let result = TrieNode {
                extension: prefix.to_string(),
                value: Some(branch_value),
                branch: vec![
                    (divergence_char.chars().next().unwrap(), Box::new(TrieNode {
                        extension: suffix.to_string(),
                        value: self.value.take(),
                        branch: self.branch.drain(..).collect(),
                    }))
                ]
            };
            *self = result;
            return
        }

        let (branch_divergence_char, branch_suffix) = branch_key.split_at(1);

        let result = TrieNode {
            extension: prefix.to_string(),
            value: None,
            branch: vec![
                (divergence_char.chars().next().unwrap(), Box::new(TrieNode {
                    extension: suffix.to_string(),
                    value: self.value.take(),
                    branch: self.branch.drain(..).collect(),
                })),
                (branch_divergence_char.chars().next().unwrap(), Box::new(TrieNode {
                    extension: branch_suffix.to_string(),
                    value: Some(branch_value),
                    branch: vec![],
                })),
            ],
        };

        *self = result;
    }

    fn insert<'a>(&mut self, key: &mut Chars<'a>, new_value: V) {
        for (i, c) in self.extension.chars().enumerate() {
            match key.next() {
                Some(seek) if seek == c => {},
                _ => return self.make_branch(i, key.as_str(), new_value),
            }
        }
            
        match key.next() {
            Some(seek) => match self.branch.binary_search_by_key(&seek, |(k, _)| *k) {
                Ok(i) => self.branch.get_mut(i).unwrap().1.insert(key, new_value),
                Err(i) => self.branch.insert(i, (seek, Box::new(TrieNode{
                    extension: key.as_str().to_string(),
                    value: Some(new_value),
                    branch: vec![],      
                }))),
            },
            None => { self.value = Some(new_value) },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut trie = Trie {
            root: TrieNode {
                extension: "".to_string(),
                value: None,
                branch: vec![],
            },
        };

        trie.insert(&"hello".to_string(), 1);
        trie.insert(&"hell".to_string(), 2);
        trie.insert(&"he".to_string(), 3);
        trie.insert(&"h".to_string(), 4);
        trie.insert(&"world".to_string(), 5);
        trie.insert(&"wor".to_string(), 6);
        trie.insert(&"wo".to_string(), 7);
        trie.insert(&"w".to_string(), 8);

        assert_eq!(trie.get(&"hello".to_string()), Some(1));
        assert_eq!(trie.get(&"hell".to_string()), Some(2));
        assert_eq!(trie.get(&"he".to_string()), Some(3));
        assert_eq!(trie.get(&"h".to_string()), Some(4));
        assert_eq!(trie.get(&"world".to_string()), Some(5));
        assert_eq!(trie.get(&"wor".to_string()), Some(6));
        assert_eq!(trie.get(&"wo".to_string()), Some(7));
        assert_eq!(trie.get(&"w".to_string()), Some(8));

        assert_eq!(trie.get(&"helloo".to_string()), None);
    }

    #[test]
    fn complex_test_trie() {

    }
}