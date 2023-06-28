// benchmark for comparing multiple map implementations, including utils/trie, std::collections::HashMap, std::collections::HashMap<S=FxHash> and std::collections::BTreeMap


extern crate test;
extern crate fxhash;

use std::collections::{HashMap, BTreeMap};
use fxhash::FxHashMap;

use test::{Bencher, black_box};
use utils::{SharedString, VectorMap, Map};

trait BenchMap {
    fn new(size: usize) -> Self;
    fn insert(&mut self, key: &SharedString);
    fn search(&mut self, key: &SharedString);
    fn delete(&mut self, key: &SharedString);
    fn drain(&mut self);
    fn clear(&mut self);
}

// Emulates statically known offset behavior
// inserted here to compare with hashmap access
struct BenchArray(Vec<u64>);

impl BenchMap for BenchArray {
    fn new(size: usize) -> Self {
        BenchArray(vec![0; size])
    }

    fn insert(&mut self, key: &SharedString) {
        let index = key.as_bytes()[0] as usize % self.0.len();
        black_box(self.0[index] = key.chars().next().unwrap() as u64);
    }

    fn search(&mut self, key: &SharedString) {
        let index = key.as_bytes()[0] as usize % self.0.len();
        let value = black_box(self.0[index]);
    }

    fn delete(&mut self, key: &SharedString) {
        let index = key.as_bytes()[0] as usize % self.0.len();
        black_box(self.0[index] = key.chars().next().unwrap() as u64);
    }

    fn drain(&mut self) {
        black_box(
        for elem in &self.0 {

        })
        //unsafe{self.0.set_len(0)}
    }

    fn clear(&mut self) {
        black_box(
            self.0.clear()
        )
    }
}
struct BenchFxHashMap(FxHashMap<SharedString, u64>);

impl BenchMap for BenchFxHashMap {
    fn new(size: usize) -> Self {
        BenchFxHashMap(FxHashMap::with_hasher(Default::default()))
    }

    fn insert(&mut self, key: &SharedString) {
        self.0.insert(key.clone(), key.chars().next().unwrap() as u64);
    }

    fn search(&mut self, key: &SharedString) {
        let data = self.0.get(&key);
        black_box(data);
    }

    fn delete(&mut self, key: &SharedString) {
        self.0.remove(&key);
    }

    fn drain(&mut self) {
        let mut elems: Vec<(SharedString, u64)> = self.0.iter().map(|(k, v)| (k.clone(), *v)).collect();
        elems.sort_unstable_by_key(|(k, _)| k.clone());
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}

struct BenchSortedVectorMap(VectorMap<u64>);

impl BenchMap for BenchSortedVectorMap {
    fn new(size: usize) -> Self {
        BenchSortedVectorMap(VectorMap::new())
    }

    fn insert(&mut self, key: &SharedString) {
        self.0.insert(key, key.chars().next().unwrap() as u64);
    }

    fn search(&mut self, key: &SharedString) {
        let data = self.0.get(key); 
        black_box(data);
    }

    fn delete(&mut self, key: &SharedString) {
        unimplemented!()
    }

    fn drain(&mut self) {
        let mut dummy = Vec::with_capacity(self.0.len());

        for (key, value) in self.0.iter() {
            dummy.push((key.clone(), value));
        }
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}

/* 
struct BenchUnsortedVectorMap(Vec<SharedString>);

impl BenchMap for BenchUnsortedVectorMap {
    fn new(size: usize) -> Self {
        BenchUnsortedVectorMap(Vec::with_capacity(size))
    }

    fn insert(&mut self, key: SharedString) {
        for elem in &self.0 {
            if elem == &key {
                return
            }
        }
        self.0.push(key)
    }

    fn search(&mut self, key: SharedString) {
        for elem in &self.0 {
            if elem == &key {
                return
            }
        } 
    }

    fn delete(&mut self, key: SharedString) {
        unimplemented!()
    }

    fn drain(&mut self) {
        for elem in &self.0 {

        }
    }
}
*/
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

fn bench_map_init<T: BenchMap>(b: &mut Bencher, n: usize, len: usize) {
    let mut map = T::new(n);
    let keys = random_keys(n, len, 0);
    let mut index = 0;

    b.iter(|| {
        let key = &keys[index];
        index = (index + 1) % n;
        map.insert(key);

        if index == 0 {
            map.clear() 
        }
    });
}

fn bench_map_insert<T: BenchMap>(b: &mut Bencher, mut map: T, n: usize, len: usize) {
    let keys = random_keys(n, len, 0);

    let mut index = 0;
    let length = keys.len();

    b.iter(|| {
        map.insert(&keys[index]);
        index = (index + 1) % length;
    });
}

fn bench_map_drain<T: BenchMap>(b: &mut Bencher, mut map: T, n: usize, len: usize) {
    let keys = random_keys(n, len, 0);

    for key in &keys {
        map.insert(key);
    }

    b.iter(|| {
        map.drain();
    });
}

fn bench_map_search<T: BenchMap>(b: &mut Bencher, mut map: T, n: usize, len: usize) {
    let keys = random_keys(n, len, 0);

    for key in &keys {
        map.insert(key);
    }

    let mut index = 0;
    let length = keys.len();

    b.iter(|| {
        map.search(&keys[index]);
        index = (index + 1) % length;
    });
}

macro_rules! bench_map {
    ($map_type_name:ident, $n:expr, $len:expr, $init_name:ident, $insert_name:ident, $search_name:ident, $drain_name: ident) => {
        #[bench]
        fn $init_name(b: &mut Bencher) {
            bench_map_init::<$map_type_name>(b, $n, $len);
        }

        #[bench]
        fn $insert_name(b: &mut Bencher) {
            let map = black_box($map_type_name::new($n));
            bench_map_insert(b, map, $n, $len);
        }

        #[bench]
        fn $search_name(b: &mut Bencher) {
            let map = black_box($map_type_name::new($n));
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
            let map = black_box($map_type_name::new($n));
            bench_map_drain(b, map, $n, $len);
        }
    };
}
/*
bench_map!(BenchTrie, 5, 5, bench_trie_insert_05_05, bench_trie_search_05_05, bench_trie_drain_05_05);
bench_map!(BenchTrie, 5, 20, bench_trie_insert_05_20, bench_trie_search_05_20, bench_trie_drain_05_20);
bench_map!(BenchTrie, 5, 80, bench_trie_insert_05_80, bench_trie_search_05_80, bench_trie_drain_05_80);

bench_map!(BenchTrie, 20, 5, bench_trie_insert_20_05, bench_trie_search_20_05, bench_trie_drain_20_05);
bench_map!(BenchTrie, 20, 20, bench_trie_insert_20_20, bench_trie_search_20_20, bench_trie_drain_20_20);
bench_map!(BenchTrie, 20, 80, bench_trie_insert_20_80, bench_trie_search_20_80, bench_trie_drain_20_80);

bench_map!(BenchTrie, 80, 5, bench_trie_insert_80_05, bench_trie_search_80_05, bench_trie_drain_80_05);
bench_map!(BenchTrie, 80, 20, bench_trie_insert_80_20, bench_trie_search_80_20, bench_trie_drain_80_20);
bench_map!(BenchTrie, 80, 80, bench_trie_insert_80_80, bench_trie_search_80_80, bench_trie_drain_80_80);



bench_map!(BenchHashMap, 5, 5, bench_hashmap_insert_05_05, bench_hashmap_search_05_05, bench_hashmap_drain_05_05);
bench_map!(BenchHashMap, 5, 20, bench_hashmap_insert_05_20, bench_hashmap_search_05_20, bench_hashmap_drain_05_20);
bench_map!(BenchHashMap, 5, 80, bench_hashmap_insert_05_80, bench_hashmap_search_05_80, bench_hashmap_drain_05_80);

bench_map!(BenchHashMap, 20, 5, bench_hashmap_insert_20_05, bench_hashmap_search_20_05, bench_hashmap_drain_20_05);
bench_map!(BenchHashMap, 20, 20, bench_hashmap_insert_20_20, bench_hashmap_search_20_20, bench_hashmap_drain_20_20);
bench_map!(BenchHashMap, 20, 80, bench_hashmap_insert_20_80, bench_hashmap_search_20_80, bench_hashmap_drain_20_80);

bench_map!(BenchHashMap, 80, 5, bench_hashmap_insert_80_05, bench_hashmap_search_80_05, bench_hashmap_drain_80_05);
bench_map!(BenchHashMap, 80, 20, bench_hashmap_insert_80_20, bench_hashmap_search_80_20, bench_hashmap_drain_80_20);
bench_map!(BenchHashMap, 80, 80, bench_hashmap_insert_80_80, bench_hashmap_search_80_80, bench_hashmap_drain_80_80);


*/
bench_map!(BenchFxHashMap, 5, 5,  bench_fxhashmap_init_005_005, bench_fxhashmap_insert_005_005, bench_fxhashmap_search_005_005, bench_fxhashmap_drain_005_005);
bench_map!(BenchFxHashMap, 5, 20, bench_fxhashmap_init_005_020, bench_fxhashmap_insert_005_020, bench_fxhashmap_search_005_020, bench_fxhashmap_drain_005_020);
bench_map!(BenchFxHashMap, 5, 80, bench_fxhashmap_init_005_080, bench_fxhashmap_insert_005_080, bench_fxhashmap_search_005_080, bench_fxhashmap_drain_005_080);
                                  
bench_map!(BenchFxHashMap, 20, 5,  bench_fxhashmap_init_020_005, bench_fxhashmap_insert_020_005, bench_fxhashmap_search_020_005, bench_fxhashmap_drain_020_005);
bench_map!(BenchFxHashMap, 20, 20, bench_fxhashmap_init_020_020, bench_fxhashmap_insert_020_020, bench_fxhashmap_search_020_020, bench_fxhashmap_drain_020_020);
bench_map!(BenchFxHashMap, 20, 80, bench_fxhashmap_init_020_080, bench_fxhashmap_insert_020_080, bench_fxhashmap_search_020_080, bench_fxhashmap_drain_020_080);
                                  
bench_map!(BenchFxHashMap, 80, 5,  bench_fxhashmap_init_080_005, bench_fxhashmap_insert_080_005, bench_fxhashmap_search_080_005, bench_fxhashmap_drain_080_005);
bench_map!(BenchFxHashMap, 80, 20, bench_fxhashmap_init_080_020, bench_fxhashmap_insert_080_020, bench_fxhashmap_search_080_020, bench_fxhashmap_drain_080_020);
bench_map!(BenchFxHashMap, 80, 80, bench_fxhashmap_init_080_080, bench_fxhashmap_insert_080_080, bench_fxhashmap_search_080_080, bench_fxhashmap_drain_080_080);
                                  
bench_map!(BenchFxHashMap, 320, 5,  bench_fxhashmap_init_320_005,  bench_fxhashmap_insert_320_005, bench_fxhashmap_search_320_005, bench_fxhashmap_drain_320_005);
bench_map!(BenchFxHashMap, 320, 20, bench_fxhashmap_init_320_020, bench_fxhashmap_insert_320_020, bench_fxhashmap_search_320_020, bench_fxhashmap_drain_320_020);
bench_map!(BenchFxHashMap, 320, 80, bench_fxhashmap_init_320_080, bench_fxhashmap_insert_320_080, bench_fxhashmap_search_320_080, bench_fxhashmap_drain_320_080);


bench_map!(BenchFxHashMap, 3200, 5,  bench_fxhashmap_init_3200_005,  bench_fxhashmap_insert_3200_005, bench_fxhashmap_search_3200_005, bench_fxhashmap_drain_3200_005);
bench_map!(BenchFxHashMap, 3200, 20, bench_fxhashmap_init_3200_020, bench_fxhashmap_insert_3200_020, bench_fxhashmap_search_3200_020, bench_fxhashmap_drain_3200_020);
bench_map!(BenchFxHashMap, 3200, 80, bench_fxhashmap_init_3200_080, bench_fxhashmap_insert_3200_080, bench_fxhashmap_search_3200_080, bench_fxhashmap_drain_3200_080);



/* 

bench_map!(BenchBTreeMap, 5, 5, bench_btreemap_insert_05_05, bench_btreemap_search_05_05, bench_btreemap_drain_05_05);
bench_map!(BenchBTreeMap, 5, 20, bench_btreemap_insert_05_20, bench_btreemap_search_05_20, bench_btreemap_drain_05_20);
bench_map!(BenchBTreeMap, 5, 80, bench_btreemap_insert_05_80, bench_btreemap_search_05_80, bench_btreemap_drain_05_80);

bench_map!(BenchBTreeMap, 20, 5, bench_btreemap_insert_20_05, bench_btreemap_search_20_05, bench_btreemap_drain_20_05);
bench_map!(BenchBTreeMap, 20, 20, bench_btreemap_insert_20_20, bench_btreemap_search_20_20, bench_btreemap_drain_20_20);
bench_map!(BenchBTreeMap, 20, 80, bench_btreemap_insert_20_80, bench_btreemap_search_20_80, bench_btreemap_drain_20_80);

bench_map!(BenchBTreeMap, 80, 5, bench_btreemap_insert_80_05, bench_btreemap_search_80_05, bench_btreemap_drain_80_05);
bench_map!(BenchBTreeMap, 80, 20, bench_btreemap_insert_80_20, bench_btreemap_search_80_20, bench_btreemap_drain_80_20);
bench_map!(BenchBTreeMap, 80, 80, bench_btreemap_insert_80_80, bench_btreemap_search_80_80, bench_btreemap_drain_80_80);
*/
/* 
bench_map!(BenchArray, 5, 5, bench_array_init_05_05, bench_array_insert_05_05, bench_array_search_05_05, bench_array_drain_05_05);
bench_map!(BenchArray, 5, 20, bench_array_init_05_20,  bench_array_insert_05_20, bench_array_search_05_20, bench_array_drain_05_20);
bench_map!(BenchArray, 5, 80, bench_array_init_05_80,  bench_array_insert_05_80, bench_array_search_05_80, bench_array_drain_05_80);
                             
bench_map!(BenchArray, 20, 5, bench_array_init_20_05,  bench_array_insert_20_05, bench_array_search_20_05, bench_array_drain_20_05);
bench_map!(BenchArray, 20, 20, bench_array_init_20_20, bench_array_insert_20_20, bench_array_search_20_20, bench_array_drain_20_20);
bench_map!(BenchArray, 20, 80, bench_array_init_20_80, bench_array_insert_20_80, bench_array_search_20_80, bench_array_drain_20_80);
                             
bench_map!(BenchArray, 80, 5, bench_array_init_80_05, bench_array_insert_80_05, bench_array_search_80_05, bench_array_drain_80_05);
bench_map!(BenchArray, 80, 20, bench_array_init_80_20, bench_array_insert_80_20, bench_array_search_80_20, bench_array_drain_80_20);
bench_map!(BenchArray, 80, 80, bench_array_init_80_80, bench_array_insert_80_80, bench_array_search_80_80, bench_array_drain_80_80);
*/
bench_map!(BenchSortedVectorMap, 5, 5, bench_sorted_vector_map_init_005_005, bench_sorted_vector_map_insert_005_005, bench_sorted_vector_map_search_005_005, bench_sorted_vector_map_drain_005_005);
bench_map!(BenchSortedVectorMap, 5, 20, bench_sorted_vector_map_init_005_020,bench_sorted_vector_map_insert_005_020, bench_sorted_vector_map_search_005_020, bench_sorted_vector_map_drain_005_020);
bench_map!(BenchSortedVectorMap, 5, 80, bench_sorted_vector_map_init_005_080,bench_sorted_vector_map_insert_005_080, bench_sorted_vector_map_search_005_080, bench_sorted_vector_map_drain_005_080);
                                        
bench_map!(BenchSortedVectorMap, 20, 5, bench_sorted_vector_map_init_020_005,bench_sorted_vector_map_insert_020_005, bench_sorted_vector_map_search_020_005, bench_sorted_vector_map_drain_020_005);
bench_map!(BenchSortedVectorMap, 20, 20, bench_sorted_vector_map_init_020_020, bench_sorted_vector_map_insert_020_020, bench_sorted_vector_map_search_020_020, bench_sorted_vector_map_drain_020_020);
bench_map!(BenchSortedVectorMap, 20, 80, bench_sorted_vector_map_init_020_080, bench_sorted_vector_map_insert_020_080, bench_sorted_vector_map_search_020_080, bench_sorted_vector_map_drain_020_080);
                                        
bench_map!(BenchSortedVectorMap, 80, 5, bench_sorted_vector_map_init_080_005,bench_sorted_vector_map_insert_080_005, bench_sorted_vector_map_search_080_005, bench_sorted_vector_map_drain_080_005);
bench_map!(BenchSortedVectorMap, 80, 20, bench_sorted_vector_map_init_080_020, bench_sorted_vector_map_insert_080_020, bench_sorted_vector_map_search_080_020, bench_sorted_vector_map_drain_080_020);
bench_map!(BenchSortedVectorMap, 80, 80, bench_sorted_vector_map_init_080_080, bench_sorted_vector_map_insert_080_080, bench_sorted_vector_map_search_080_080, bench_sorted_vector_map_drain_080_080);
                                        
bench_map!(BenchSortedVectorMap, 320, 5, bench_sorted_vector_map_init_320_005, bench_sorted_vector_map_insert_320_005, bench_sorted_vector_map_search_320_005, bench_sorted_vector_map_drain_320_005);
bench_map!(BenchSortedVectorMap, 320, 20, bench_sorted_vector_map_init_320_02, bench_sorted_vector_map_insert_320_020, bench_sorted_vector_map_search_320_020, bench_sorted_vector_map_drain_320_020);
bench_map!(BenchSortedVectorMap, 320, 80, bench_sorted_vector_map_init_320_08, bench_sorted_vector_map_insert_320_080, bench_sorted_vector_map_search_320_080, bench_sorted_vector_map_drain_320_080);

bench_map!(BenchSortedVectorMap, 3200, 5, bench_sorted_vector_map_init_3200_005, bench_sorted_vector_map_insert_3200_005, bench_sorted_vector_map_search_3200_005, bench_sorted_vector_map_drain_3200_005);
bench_map!(BenchSortedVectorMap, 3200, 20, bench_sorted_vector_map_init_3200_02, bench_sorted_vector_map_insert_3200_020, bench_sorted_vector_map_search_3200_020, bench_sorted_vector_map_drain_3200_020);
bench_map!(BenchSortedVectorMap, 3200, 80, bench_sorted_vector_map_init_3200_08, bench_sorted_vector_map_insert_3200_080, bench_sorted_vector_map_search_3200_080, bench_sorted_vector_map_drain_3200_080);


/* 
bench_map!(BenchUnsortedVectorMap, 5, 5, bench_unsorted_vector_map_insert_05_05, bench_unsorted_vector_map_search_05_05, bench_unsorted_vector_map_drain_05_05);
bench_map!(BenchUnsortedVectorMap, 5, 20, bench_unsorted_vector_map_insert_05_20, bench_unsorted_vector_map_search_05_20, bench_unsorted_vector_map_drain_05_20);
bench_map!(BenchUnsortedVectorMap, 5, 80, bench_unsorted_vector_map_insert_05_80, bench_unsorted_vector_map_search_05_80, bench_unsorted_vector_map_drain_05_80);

bench_map!(BenchUnsortedVectorMap, 20, 5, bench_unsorted_vector_map_insert_20_05, bench_unsorted_vector_map_search_20_05, bench_unsorted_vector_map_drain_20_05);
bench_map!(BenchUnsortedVectorMap, 20, 20, bench_unsorted_vector_map_insert_20_20, bench_unsorted_vector_map_search_20_20, bench_unsorted_vector_map_drain_20_20);
bench_map!(BenchUnsortedVectorMap, 20, 80, bench_unsorted_vector_map_insert_20_80, bench_unsorted_vector_map_search_20_80, bench_unsorted_vector_map_drain_20_80);

bench_map!(BenchUnsortedVectorMap, 80, 5, bench_unsorted_vector_map_insert_80_05, bench_unsorted_vector_map_search_80_05, bench_unsorted_vector_map_drain_80_05);
bench_map!(BenchUnsortedVectorMap, 80, 20, bench_unsorted_vector_map_insert_80_20, bench_unsorted_vector_map_search_80_20, bench_unsorted_vector_map_drain_80_20);
bench_map!(BenchUnsortedVectorMap, 80, 80, bench_unsorted_vector_map_insert_80_80, bench_unsorted_vector_map_search_80_80, bench_unsorted_vector_map_drain_80_80);
*/