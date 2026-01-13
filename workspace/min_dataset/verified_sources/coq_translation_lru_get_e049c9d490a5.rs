use vstd::prelude::*;

verus! {

pub struct LRUCache { pub items: Seq<(nat, nat)>, pub capacity: nat }


pub open spec fn lru_get(cache: LRUCache, key: nat) -> Option<nat> decreases cache.items.len() {
    if cache.items.len() == 0 { None }
    else if cache.items[0].0 == key { Some(cache.items[0].1) }
    else { lru_get(LRUCache { items: cache.items.skip(1), capacity: cache.capacity }, key) }
}

} // verus!