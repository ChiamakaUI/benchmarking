use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    sync::{Arc, Mutex, RwLock},
};

use dashmap::DashMap;
use rayon::prelude::*;
// use flurry::HashMap as FlurryMap;

pub fn vec_simple_push(max: usize){
    let mut benchmap: Vec<usize> = Vec::new();

    for num in 0..max {
        benchmap.push(num);
    }
}

pub fn vec_prealloc_push(max: usize){
    let mut benchmap: Vec<usize> = Vec::with_capacity(max);

    for num in 0..max {
        benchmap.push(num);
    }
}

pub fn vec_simple_insert(max: usize) {
    let mut benchmap = Vec::new();

    for num in 0..max {
        benchmap.insert(num, num);
    }
}

pub fn vec_prealloc_insert(max: usize) {
    let mut benchmap = Vec::with_capacity(max);

    for num in 0..max {
        benchmap.insert(num, num);
    }
}

pub fn hashset_simple(max: usize) {
    let mut benchmap = HashSet::new();

    for num in 0..max {
        benchmap.insert(num);
    }
}

pub fn hashset_prealloc(max: usize) {
    let mut benchmap = HashSet::with_capacity(max);

    for num in 0..max {
        benchmap.insert(num);
    }
}

pub fn vecdeque_simple(max: usize) {
    let mut benchmap = VecDeque::new();

    for num in 0..max {
        benchmap.push_front(num);
    }
}

pub fn vecdeque_prealloc(max: usize) {
    let mut benchmap = VecDeque::with_capacity(max);

    for num in 0..max {
        benchmap.push_front(num);
    }
}

pub fn vecdeque_simple_insert(max: usize) {
    let mut benchmap = VecDeque::new();

    for num in 0..max {
        benchmap.insert(0, num);
    }
}


pub fn hashmap_simple(max: usize){
    let mut benchmap = HashMap::new();
    for num in 0..max {
        benchmap.insert(num, num);
    }
}

pub fn hashmap_prealloc(max: usize){
    let mut benchmap = HashMap::with_capacity(max);
    for num in 0..max {
        benchmap.insert(num,num);
    }
}

pub fn dashmap_simple(max: usize){
    let benchmap = DashMap::new();

    for num in 0..max {
        benchmap.insert(num, num);
    }
}

pub fn dashmap_prealloc(max: usize){
    let benchmap = DashMap::with_capacity(max);

    for num in 0..max {
        benchmap.insert(num, num);
    }
}


pub fn btreemap_simple(max: usize) {
    let mut benchmap = BTreeMap::new();

    for num in 0..max {
        benchmap.insert(num, num);
    }
}

pub fn hashmap_rwlock_write(max: usize) {
    let benchmap = RwLock::new(HashMap::new());

    (0..max).into_par_iter().for_each(|n| {
        benchmap.write().unwrap().insert(n, n);
    });
}

pub fn hashmap_rwlock_read(max: usize) {
    let mut initial_map = HashMap::new();

    for num in 0..max {
        initial_map.insert(num, num);
    }

    let benchmap: RwLock<HashMap<usize, usize>> = RwLock::new(initial_map);

    (0..max).into_par_iter().for_each(|n| {
        let guard = benchmap.read().unwrap();
        let _ = guard.get(&n);
    });

}

pub fn hashmap_threaded(max: usize) {
    let benchmap = Arc::new(Mutex::new(HashMap::new()));

    (0..max).into_par_iter().for_each(|n|{
        benchmap.lock().unwrap().insert(n, n);
    });
}


pub fn dashmap_threaded(max: usize) {
    let benchmap = DashMap::new();

    (0..max).into_par_iter().for_each(|n|{
        benchmap.insert(n, n);
    });

}

// pub fn flurrymap_threaded(max: usize){
//     let benchmark = FlurryMap::new();

//     (0..max).into_par_iter().for_each(|n|{
//         benchmark.insert(n, n, guard);
//     });
// }

// VEC, hashmaps, dashmaps, mutext, rwlocks, hashsets, vecdeque