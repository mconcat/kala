// benchmark for comparing multiple map implementations, including utils/trie, std::collections::HashMap, std::collections::HashMap<S=FxHash> and std::collections::BTreeMap


extern crate test;
extern crate fxhash;

use std::collections::{HashMap, BTreeMap};
use fxhash::FxHashMap;

use test::{Bencher};
use utils::{trie, SharedString};

trait BenchMap {
    fn new() -> Self;
    fn insert(&mut self, key: SharedString);
    fn search(&self, key: SharedString);
    fn delete(&mut self, key: SharedString);
    fn drain(&mut self);
}

struct BenchTrie(trie::Trie<()>);

impl BenchMap for BenchTrie {
    fn new() -> Self {
        BenchTrie(trie::Trie::empty())
    }

    fn insert(&mut self, key: SharedString) {
        self.0.insert(&key, ());
    }

    fn search(&self, key: SharedString) {
        self.0.get(&key);
    }

    fn delete(&mut self, key: SharedString) {
        self.0.insert(&key, ());
    }

    fn drain(&mut self) {
        self.0 = trie::Trie::empty();
    }
}

struct BenchHashMap(HashMap<SharedString, ()>);

impl BenchMap for BenchHashMap {
    fn new() -> Self {
        BenchHashMap(HashMap::new())
    }

    fn insert(&mut self, key: SharedString) {
        self.0.insert(key, ());
    }

    fn search(&self, key: SharedString) {
        self.0.get(&key);
    }

    fn delete(&mut self, key: SharedString) {
        self.0.remove(&key);
    }

    fn drain(&mut self) {
        self.0 = HashMap::new();
    }
}

struct BenchFxHashMap(FxHashMap<SharedString, ()>);

impl BenchMap for BenchFxHashMap {
    fn new() -> Self {
        BenchFxHashMap(FxHashMap::default())
    }

    fn insert(&mut self, key: SharedString) {
        self.0.insert(key, ());
    }

    fn search(&self, key: SharedString) {
        self.0.get(&key);
    }

    fn delete(&mut self, key: SharedString) {
        self.0.remove(&key);
    }

    fn drain(&mut self) {
        self.0 = FxHashMap::default();
    }
}

struct BenchBTreeMap(BTreeMap<SharedString, ()>);

impl BenchMap for BenchBTreeMap {
    fn new() -> Self {
        BenchBTreeMap(BTreeMap::new())
    }

    fn insert(&mut self, key: SharedString) {
        self.0.insert(key, ());
    }

    fn search(&self, key: SharedString) {
        self.0.get(&key);
    }

    fn delete(&mut self, key: SharedString) {
        self.0.remove(&key);
    }

    fn drain(&mut self) {
        self.0 = BTreeMap::new();
    }
}

fn random_keys(n: usize, len: usize, seed: u64) -> Vec<SharedString> {
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;

    let mut keys = Vec::with_capacity(n);

    for _ in 0..n {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect();
        keys.push(SharedString::from_string(rand_string));
    }

    keys
}

fn bench_map_insert<T: BenchMap>(b: &mut Bencher, mut map: T, n: usize, len: usize) {
    let keys = random_keys(n, len, 0);

    b.iter(|| {
        for key in keys {
            map.insert(key);
        }
    });
}

fn bench_map_drain<T: BenchMap>(b: &mut Bencher, mut map: T, n: usize, len: usize) {
    let keys = random_keys(n, len, 0);

    for key in keys {
        map.insert(key);
    }

    b.iter(|| {
        map.drain()
    });
}

fn bench_map_search<T: BenchMap>(b: &mut Bencher, mut map: T, n: usize, len: usize) {
    let keys = random_keys(n, len, 0);

    for key in keys {
        map.insert(key);
    }

    b.iter(|| {
        for key in keys {
            map.search(key);
        }
    });
}

macro_rules! bench_map {
    ($map_type_name:ident, $n:expr, $len:expr, $insert_name:ident, $search_name:ident, $drain_name: ident) => {
        #[bench]
        fn $insert_name(b: &mut Bencher) {
            let map = $map_type_name::new();
            bench_map_insert(b, map, $n, $len);
        }

        #[bench]
        fn $search_name(b: &mut Bencher) {
            let map = $map_type_name::new();
            bench_map_search(b, map, $n, $len);
        }

/* 
        #[bench]
        fn bench_($map_type_name)_delete_($n)_($len)(b: &mut Bencher) {
            let map = $map_type_name::new();
            bench_map_insert(b, map, $n, $len);
        }
*/
        #[bench]
        fn $drain_name(b: &mut Bencher) {
            let map = $map_type_name::new();
            bench_map_drain(b, map, $n, $len);
        }
    };
}


bench_map!(BenchTrie, 5, 5, bench_trie_insert_5_5, bench_trie_search_5_5, bench_trie_drain_5_5);
bench_map!(BenchTrie, 5, 20, bench_trie_insert_5_20, bench_trie_search_5_20, bench_trie_drain_5_20);
bench_map!(BenchTrie, 5, 80, bench_trie_insert_5_80, bench_trie_search_5_80, bench_trie_drain_5_80);

bench_map!(BenchTrie, 20, 5, bench_trie_insert_20_5, bench_trie_search_20_5, bench_trie_drain_20_5);
bench_map!(BenchTrie, 20, 20, bench_trie_insert_20_20, bench_trie_search_20_20, bench_trie_drain_20_20);
bench_map!(BenchTrie, 20, 80, bench_trie_insert_20_80, bench_trie_search_20_80, bench_trie_drain_20_80);

bench_map!(BenchTrie, 80, 5, bench_trie_insert_80_5, bench_trie_search_80_5, bench_trie_drain_80_5);
bench_map!(BenchTrie, 80, 20, bench_trie_insert_80_20, bench_trie_search_80_20, bench_trie_drain_80_20);
bench_map!(BenchTrie, 80, 80, bench_trie_insert_80_80, bench_trie_search_80_80, bench_trie_drain_80_80);



bench_map!(BenchHashMap, 5, 5, bench_hashmap_insert_5_5, bench_hashmap_search_5_5, bench_hashmap_drain_5_5);
bench_map!(BenchHashMap, 5, 20, bench_hashmap_insert_5_20, bench_hashmap_search_5_20, bench_hashmap_drain_5_20);
bench_map!(BenchHashMap, 5, 80, bench_hashmap_insert_5_80, bench_hashmap_search_5_80, bench_hashmap_drain_5_80);

bench_map!(BenchHashMap, 20, 5, bench_hashmap_insert_20_5, bench_hashmap_search_20_5, bench_hashmap_drain_20_5);
bench_map!(BenchHashMap, 20, 20, bench_hashmap_insert_20_20, bench_hashmap_search_20_20, bench_hashmap_drain_20_20);
bench_map!(BenchHashMap, 20, 80, bench_hashmap_insert_20_80, bench_hashmap_search_20_80, bench_hashmap_drain_20_80);

bench_map!(BenchHashMap, 80, 5, bench_hashmap_insert_80_5, bench_hashmap_search_80_5, bench_hashmap_drain_80_5);
bench_map!(BenchHashMap, 80, 20, bench_hashmap_insert_80_20, bench_hashmap_search_80_20, bench_hashmap_drain_80_20);
bench_map!(BenchHashMap, 80, 80, bench_hashmap_insert_80_80, bench_hashmap_search_80_80, bench_hashmap_drain_80_80);



bench_map!(BenchFxHashMap, 5, 5, bench_fxhashmap_insert_5_5, bench_fxhashmap_search_5_5, bench_fxhashmap_drain_5_5);
bench_map!(BenchFxHashMap, 5, 20, bench_fxhashmap_insert_5_20, bench_fxhashmap_search_5_20, bench_fxhashmap_drain_5_20);
bench_map!(BenchFxHashMap, 5, 80, bench_fxhashmap_insert_5_80, bench_fxhashmap_search_5_80, bench_fxhashmap_drain_5_80);

bench_map!(BenchFxHashMap, 20, 5, bench_fxhashmap_insert_20_5, bench_fxhashmap_search_20_5, bench_fxhashmap_drain_20_5);
bench_map!(BenchFxHashMap, 20, 20, bench_fxhashmap_insert_20_20, bench_fxhashmap_search_20_20, bench_fxhashmap_drain_20_20);
bench_map!(BenchFxHashMap, 20, 80, bench_fxhashmap_insert_20_80, bench_fxhashmap_search_20_80, bench_fxhashmap_drain_20_80);

bench_map!(BenchFxHashMap, 80, 5, bench_fxhashmap_insert_80_5, bench_fxhashmap_search_80_5, bench_fxhashmap_drain_80_5);
bench_map!(BenchFxHashMap, 80, 20, bench_fxhashmap_insert_80_20, bench_fxhashmap_search_80_20, bench_fxhashmap_drain_80_20);
bench_map!(BenchFxHashMap, 80, 80, bench_fxhashmap_insert_80_80, bench_fxhashmap_search_80_80, bench_fxhashmap_drain_80_80);




bench_map!(BenchBTreeMap, 5, 5, bench_btreemap_insert_5_5, bench_btreemap_search_5_5, bench_btreemap_drain_5_5);
bench_map!(BenchBTreeMap, 5, 20, bench_btreemap_insert_5_20, bench_btreemap_search_5_20, bench_btreemap_drain_5_20);
bench_map!(BenchBTreeMap, 5, 80, bench_btreemap_insert_5_80, bench_btreemap_search_5_80, bench_btreemap_drain_5_80);

bench_map!(BenchBTreeMap, 20, 5, bench_btreemap_insert_20_5, bench_btreemap_search_20_5, bench_btreemap_drain_20_5);
bench_map!(BenchBTreeMap, 20, 20, bench_btreemap_insert_20_20, bench_btreemap_search_20_20, bench_btreemap_drain_20_20);
bench_map!(BenchBTreeMap, 20, 80, bench_btreemap_insert_20_80, bench_btreemap_search_20_80, bench_btreemap_drain_20_80);

bench_map!(BenchBTreeMap, 80, 5, bench_btreemap_insert_80_5, bench_btreemap_search_80_5, bench_btreemap_drain_80_5);
bench_map!(BenchBTreeMap, 80, 20, bench_btreemap_insert_80_20, bench_btreemap_search_80_20, bench_btreemap_drain_80_20);
bench_map!(BenchBTreeMap, 80, 80, bench_btreemap_insert_80_80, bench_btreemap_search_80_80, bench_btreemap_drain_80_80);