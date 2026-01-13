use vstd::prelude::*;

verus! {

pub enum Treap {
    E,
    T { key: int, priority: nat, left: Box<Treap>, right: Box<Treap> },
}


pub open spec fn forall_gt(t: Treap, bound: int) -> bool decreases t {
    match t { Treap::E => true, Treap::T { key, left, right, .. } => key > bound && forall_gt(*left, bound) && forall_gt(*right, bound) }
}

} // verus!