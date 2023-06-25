

extern crate fxhash;
use std::{path::Iter, fmt::{Debug, Formatter}};

extern crate alloc;

use fxhash::FxHashMap;

use crate::{SharedString, OwnedSlice};

// Common map trait for sorted vector and fxhashmap
pub trait Map<V>: Sized {
    type MapIterator: Iterator<Item = (SharedString, V)>;

    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn get(&mut self, key: &SharedString) -> Option<&mut V>;
    fn insert(&mut self, key: &SharedString, value: V) -> Option<V>;
    fn drain(&mut self) -> Self::MapIterator;
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

    fn get(&mut self, key: &SharedString) -> Option<&mut V> {
        self.0.get_mut(key)
    }

    fn insert(&mut self, key: &SharedString, value: V) -> Option<V> {
        self.0.insert(key.clone(), value)
    }

    fn drain(&mut self) -> Self::MapIterator {
        let mut elements: Vec<(SharedString, V)> = self.0.drain().collect();
        elements.sort_by_key(|(key, _)| key.clone());
        HashMapIterator{
            elements: OwnedSlice::from_vec(elements),
            cursor: 0,
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

pub struct HashMapIterator<V>{
    elements: OwnedSlice<(SharedString, V)>,
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

pub struct VectorMap<V>{
    keys: Vec<SharedString>,
    values: Vec<V>,
}

impl<V: Clone> Map<V> for VectorMap<V> {
    type MapIterator = VectorMapIterator<V>;

    fn new() -> Self {
        Self{
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self{
            keys: Vec::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
        }
    }

    fn get(&mut self, key: &SharedString) -> Option<&mut V> {
        match self.keys.binary_search(key) {
            Ok(i) => Some(&mut self.values[i]),
            Err(_) => None,
        }
    }

    fn insert(&mut self, key: &SharedString, value: V) -> Option<V> {
        match self.keys.binary_search(key) {
            Ok(i) => {
                Some(std::mem::replace(&mut self.values[i], value))
            },
            Err(i) => {
                self.keys.insert(i, key.clone());
                self.values.insert(i, value);
                None
            },
        }
    }

    fn drain(&mut self) -> Self::MapIterator {
        let keys = self.keys.drain(..);
        let values = self.values.drain(..);
        
        VectorMapIterator {
            keys: keys.collect(),
            values: values.collect(),
            cursor: 0,
        }
    }

    fn len(&self) -> usize {
        self.keys.len()
    }
}

pub struct VectorMapIterator<V>{
    keys: Vec<SharedString>,
    values: Vec<V>,
    cursor: usize,
}

impl<V: Clone> Iterator for VectorMapIterator<V> {
    type Item = (SharedString, V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor == self.keys.len() {
            None
        } else {
            let item = (self.keys[self.cursor].clone(), self.values[self.cursor].clone());
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