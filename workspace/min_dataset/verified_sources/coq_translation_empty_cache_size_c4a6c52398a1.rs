use vstd::prelude::*;

verus! {

pub struct LRUCache { pub items: Seq<(nat, nat)>, pub capacity: nat }

pub open spec fn lru_empty(cap: nat) -> LRUCache { LRUCache { items: Seq::empty(), capacity: cap } }

pub open spec fn lru_size(cache: LRUCache) -> nat { cache.items.len() }


pub proof fn empty_cache_size(cap: nat) ensures lru_size(lru_empty(cap)) == 0 {}

} // verus!