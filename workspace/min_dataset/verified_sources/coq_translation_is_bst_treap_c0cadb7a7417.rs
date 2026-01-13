use vstd::prelude::*;

verus! {

pub enum Treap {
    E,
    T { key: int, priority: nat, left: Box<Treap>, right: Box<Treap> },
}

pub open spec fn forall_lt(t: Treap, bound: int) -> bool decreases t {
    match t { Treap::E => true, Treap::T { key, left, right, .. } => key < bound && forall_lt(*left, bound) && forall_lt(*right, bound) }
}

pub open spec fn forall_gt(t: Treap, bound: int) -> bool decreases t {
    match t { Treap::E => true, Treap::T { key, left, right, .. } => key > bound && forall_gt(*left, bound) && forall_gt(*right, bound) }
}


pub open spec fn is_bst_treap(t: Treap) -> bool decreases t {
    match t {
        Treap::E => true,
        Treap::T { key, left, right, .. } =>
            forall_lt(*left, key) && forall_gt(*right, key) && is_bst_treap(*left) && is_bst_treap(*right)
    }
}

} // verus!