use vstd::prelude::*;

verus! {

pub enum Treap {
    E,
    T { key: int, priority: nat, left: Box<Treap>, right: Box<Treap> },
}

pub open spec fn priority_ge(t: Treap, bound: nat) -> bool {
    match t { Treap::E => true, Treap::T { priority, .. } => priority <= bound }
}


pub open spec fn is_heap_treap(t: Treap) -> bool decreases t {
    match t {
        Treap::E => true,
        Treap::T { priority, left, right, .. } =>
            priority_ge(*left, priority) && priority_ge(*right, priority) && is_heap_treap(*left) && is_heap_treap(*right)
    }
}

} // verus!