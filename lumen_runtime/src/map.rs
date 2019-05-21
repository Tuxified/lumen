use std::cmp::Ordering::{self, *};

use im::hashmap::{HashMap, Iter};

use crate::heap::{CloneIntoHeap, Heap};
use crate::integer::Integer;
use crate::term::{Tag, Term};

pub struct Map {
    #[allow(dead_code)]
    header: Term,
    inner: HashMap<Term, Term>,
}

impl Map {
    pub fn new(inner: HashMap<Term, Term>) -> Self {
        Map {
            header: Term {
                tagged: Tag::Map as usize,
            },
            inner,
        }
    }

    pub fn get(&self, key: Term) -> Option<Term> {
        self.inner.get(&key).map(|ref_value| ref_value.clone())
    }

    pub fn iter(&self) -> Iter<Term, Term> {
        self.inner.iter()
    }

    pub fn is_key(&self, key: Term) -> bool {
        self.inner.contains_key(&key)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn size(&self) -> Integer {
        self.len().into()
    }

    // Private

    fn sorted_keys(&self) -> Vec<Term> {
        let mut key_vec: Vec<Term> = Vec::new();
        key_vec.extend(self.inner.keys());
        key_vec.sort_unstable_by(|key1, key2| key1.cmp(&key2));

        key_vec
    }
}

impl CloneIntoHeap for &'static Map {
    fn clone_into_heap(&self, heap: &Heap) -> &'static Map {
        let mut heap_inner: HashMap<Term, Term> = HashMap::new();

        for (key, value) in &self.inner {
            heap_inner.insert(key.clone_into_heap(heap), value.clone_into_heap(heap));
        }

        heap.im_hash_map_to_map(heap_inner)
    }
}

impl Eq for Map {}

impl Ord for Map {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for Map {
    fn eq(&self, other: &Map) -> bool {
        match self.len().eq(&other.len()) {
            true => {
                let self_key_vec = self.sorted_keys();
                let other_key_vec = other.sorted_keys();

                match self_key_vec.eq(&other_key_vec) {
                    true => {
                        let self_inner = &self.inner;
                        let other_inner = &other.inner;

                        self_key_vec.iter().all(|key| {
                            self_inner
                                .get(&key)
                                .unwrap()
                                .eq(other_inner.get(&key).unwrap())
                        })
                    }
                    eq => eq,
                }
            }
            eq => eq,
        }
    }
}

impl PartialOrd for Map {
    /// > * Maps are compared by size, then by keys in ascending term order,
    /// >   then by values in key order.   In the specific case of maps' key
    /// >   ordering, integers are always considered to be less than floats.
    fn partial_cmp(&self, other: &Map) -> Option<Ordering> {
        match self.len().partial_cmp(&other.len()) {
            Some(Equal) => {
                let self_key_vec = self.sorted_keys();
                let other_key_vec = other.sorted_keys();

                match self_key_vec.partial_cmp(&other_key_vec) {
                    Some(Equal) => {
                        let self_inner = &self.inner;
                        let other_inner = &other.inner;
                        let mut final_partial_ordering = Some(Equal);

                        for key in self_key_vec {
                            match self_inner
                                .get(&key)
                                .unwrap()
                                .partial_cmp(other_inner.get(&key).unwrap())
                            {
                                Some(Equal) => continue,
                                partial_ordering => {
                                    final_partial_ordering = partial_ordering;

                                    break;
                                }
                            }
                        }

                        final_partial_ordering
                    }
                    partial_ordering => partial_ordering,
                }
            }
            partial_ordering => partial_ordering,
        }
    }
}