use std::{rc::Rc, fmt::{Formatter, Debug}};

use fxhash::FxHashMap;

pub type Map<V> = FxHashMap<Rc<str>, V>;
pub struct MapPool<V>(Vec<Map<V>>);

impl<V> Debug for MapPool<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HashMapPool")
            .field("len", &self.0.len())
            .finish()
    }
}

impl<V> MapPool<V> {
    pub fn new() -> Self {
        MapPool(vec![])
    }

    pub fn get(&mut self) -> Map<V> {
        if self.0.is_empty() {
            Map::default()
        } else {
            self.0.pop().unwrap()
        }
    }

    pub fn drain(&mut self, mut map: Map<V>) -> Vec<(Rc<str>, V)> {
        let res = map.drain().collect();

        self.0.push(map);

        res
    }
}