

extern crate fxhash;
use std::{path::Iter, fmt::{Debug, Formatter}};

extern crate alloc;

use fxhash::FxHashMap;

use crate::{SharedString};

// Common map trait for sorted vector and fxhashmap
pub trait Map<V>: Sized {
    type MapIterator: Iterator<Item = (SharedString, V)>;

    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn get(&mut self, key: SharedString) -> Option<&mut V>;
    fn insert(&mut self, key: SharedString, value: V) -> Option<V>;
    fn iter(&self) -> Self::MapIterator;
    fn drain(&mut self) -> Self::MapIterator;
    fn clear(&mut self);
    fn len(&self) -> usize;
}

pub struct FxMap<V>(FxHashMap<SharedString, V>);

impl<V: Debug> Debug for FxMap<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.0.iter()).finish()
    }
}

impl<V: Clone> Map<V> for FxMap<V> {
    type MapIterator = HashMapIterator<V>;

    fn new() -> Self {
        Self(FxHashMap::default())
    }

    fn with_capacity(capacity: usize) -> Self {
        Self(FxHashMap::with_capacity_and_hasher(capacity, Default::default()))
    }

    fn get(&mut self, key: SharedString) -> Option<&mut V> {
        self.0.get_mut(&key)
    }

    fn insert(&mut self, key: SharedString, value: V) -> Option<V> {
        self.0.insert(key.clone(), value)
    }

    fn iter(&self) -> Self::MapIterator {
        let mut elements: Vec<(SharedString, V)> = self.0.iter().map(|(key, value)| (key.clone(), value.clone())).collect();
        elements.sort_unstable_by_key(|(key, _)| key.clone());
        HashMapIterator{
            elements,
            cursor: 0,
        }
    }

    fn drain(&mut self) -> Self::MapIterator {
        let mut elements: Vec<(SharedString, V)> = self.0.drain().collect();
        elements.sort_unstable_by_key(|(key, _)| key.clone());
        HashMapIterator{
            elements,
            cursor: 0,
        }
    }

    fn clear(&mut self) {
        self.0.clear()
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

pub struct HashMapIterator<V>{
    elements: Vec<(SharedString, V)>,
    cursor: usize,
}

impl<V: Clone> Iterator for HashMapIterator<V> {
    type Item = (SharedString, V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor == self.elements.len() {
            None
        } else {
            let item = self.elements[self.cursor].clone();
            self.cursor += 1;
            Some(item)
        }
    }
}

pub struct VectorMap<V, const limit: usize = 16>{
    pairs: Vec<(SharedString, V)>,
}

impl<V: Debug+Clone, const limit: usize> Debug for VectorMap<V, limit> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.pairs.iter().map(|(k, v)| (k.clone(), v.clone()))).finish()
    }
}

impl<V, const limit: usize> VectorMap<V, limit> {
    fn to_be_sorted(&self) -> bool {
        self.pairs.len() > limit
    }

    fn get_sorted(&mut self, key: SharedString) -> Option<&mut V> {
        match self.pairs.binary_search_by_key(&key, |(key, _)| key.clone()) {
            Ok(i) => Some(&mut self.pairs[i].1),
            Err(_) => None,
        }
    }

    fn insert_sorted(&mut self, key: SharedString, value: V) -> Option<V> {
        match self.pairs.binary_search_by_key(&key, |(key, _)| key.clone()) {
            Ok(i) => {
                Some(std::mem::replace(&mut self.pairs[i].1, value))
            },
            Err(i) => {
                self.pairs.insert(i, (key.clone(), value));
                None
            },
        }
    }

    fn drain_sorted(&mut self) -> VectorMapIterator<V> {
        let pairs = self.pairs.drain(..).collect(); 
        
        VectorMapIterator {
            pairs,
            cursor: 0,
        }
    }

    fn get_unsorted(&mut self, key: SharedString) -> Option<&mut V> {
        for (i, (key, value)) in self.pairs.iter_mut().enumerate() {
            if key == key {
                return Some(value);
            }
        }
        None
    }

    fn insert_unsorted(&mut self, key: SharedString, new_value: V) -> Option<V> {
        for (i, (key, value)) in self.pairs.iter_mut().enumerate() {
            if key == key {
                return Some(std::mem::replace(value, new_value));
            }
        }
        self.pairs.push((key.clone(), new_value));

        if self.to_be_sorted() {
            self.pairs.sort_unstable_by_key(|(key, _)| key.clone());
        }

        None
    }

    fn drain_unsorted(&mut self) -> VectorMapIterator<V> {
        let mut pairs: Vec<(SharedString, V)> = self.pairs.drain(..).collect();

        pairs.sort_unstable_by_key(|(key, _)| key.clone());

        VectorMapIterator {
            pairs,
            cursor: 0,
        }
    }
}


impl<V: Clone, const limit: usize> Map<V> for VectorMap<V, limit> {
    type MapIterator = VectorMapIterator<V>;

    fn new() -> Self {
        Self{
            pairs: Vec::new(),
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self{
            pairs: Vec::with_capacity(capacity),
        }
    }

    fn get(&mut self, key: SharedString) -> Option<&mut V> {
        if self.to_be_sorted() {
            self.get_sorted(key)
        } else {
            self.get_unsorted(key)
        } 
    }

    fn insert(&mut self, key: SharedString, value: V) -> Option<V> {
        if self.to_be_sorted() {
            self.insert_sorted(key, value)
        } else {
            self.insert_unsorted(key, value)
        }
    }

    fn iter(&self) -> Self::MapIterator {
        if self.to_be_sorted() {
            let mut elements: Vec<(SharedString, V)> = self.pairs.iter().map(|(key, value)| (key.clone(), value.clone())).collect();
            elements.sort_unstable_by_key(|(key, _)| key.clone());
            VectorMapIterator{
                pairs: elements,
                cursor: 0,
            }
        } else {
            VectorMapIterator{
                pairs: self.pairs.clone(),
                cursor: 0,
            }
        }
    }

    fn drain(&mut self) -> Self::MapIterator {
        if self.to_be_sorted() {
            self.drain_sorted()
        } else {
            self.drain_unsorted()
        }
    }

    fn clear(&mut self) {
        self.pairs.clear()
    }

    fn len(&self) -> usize {
        self.pairs.len()
    }
}

pub struct VectorMapIterator<V>{
    pairs: Vec<(SharedString, V)>,
    cursor: usize,
}

impl<V: Clone> Iterator for VectorMapIterator<V> {
    type Item = (SharedString, V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor == self.pairs.len() {
            None
        } else {
            let item = self.pairs[self.cursor].clone();
            self.cursor += 1;
            Some(item)
        }
    }
}
/* 
pub struct SortedMap<V> {
    map: Map<V>,
    sort: Vec<SharedString>,
}

// re-export HashMap methods
impl<V> SortedMap<V> {
    pub fn new() -> Self {
        Self {
            map: FxHashMap::default(),
            sort: Vec::new(),
        }
    }

    pub fn get(&self, key: SharedString) -> Option<&V> {
        self.map.get(&key)
    }

    pub fn insert(&mut self, key: SharedString, value: V) -> Option<V> {
        let old = self.map.insert(key.clone(), value);
        if old.is_none() {
            self.sort.push(key)
        }
        old
    }

    pub fn 
}
*/
pub trait MapPool<V> {
    type Map: Map<V>;

    fn new() -> Self;
    fn get(&mut self) -> Self::Map;
    fn drain(&mut self, map: Self::Map) -> <Self::Map as Map<V>>::MapIterator;
}

pub struct VectorMapPool<V>(Vec<VectorMap<V>>);

impl<V> Debug for VectorMapPool<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HashMapPool")
            .field("len", &self.0.len())
            .finish()
    }
}

impl<V: Clone> MapPool<V> for VectorMapPool<V> {
    type Map = VectorMap<V>;

    fn new() -> Self {
        VectorMapPool(vec![])
    }

    fn get(&mut self) -> Self::Map {
        if self.0.is_empty() {
            VectorMap::new()
        } else {
            self.0.pop().unwrap()
        }
    }

    fn drain(&mut self, mut map: Self::Map) -> VectorMapIterator<V> {
        let res = map.drain();

        self.0.push(map);

        res
    }
}

pub struct FxMapPool<V>(Vec<FxMap<V>>);

impl<V> Debug for FxMapPool<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HashMapPool")
            .field("len", &self.0.len())
            .finish()
    }
}

impl<V: Clone> MapPool<V> for FxMapPool<V> {
    type Map = FxMap<V>;

    fn new() -> Self {
        FxMapPool(vec![])
    }

    fn get(&mut self) -> Self::Map {
        if self.0.is_empty() {
            FxMap::new()
        } else {
            self.0.pop().unwrap()
        }
    }

    fn drain(&mut self, mut map: Self::Map) -> HashMapIterator<V> {
        let res = map.drain();

        self.0.push(map);

        res
    }
}
/* 

#[derive(Debug, PartialEq, Clone)]
pub struct VectorMapPool<M>(Vec<VectorMap<V>>);

impl<V> VectorMapPool<V> {
    pub fn new() -> Self {
        VectorMapPool(vec![])
    }

    pub fn get(&mut self) -> M {
        if self.0.is_empty() {
            FxHashMap::default()
        } else {
            self.0.pop().unwrap()
        }
    }

    pub fn drain<'a>(&mut self, mut map: M) -> Vec<(SharedString, V)> {
        let res = map.drain().collect();

        self.0.push(map);

        res
    }
}*/