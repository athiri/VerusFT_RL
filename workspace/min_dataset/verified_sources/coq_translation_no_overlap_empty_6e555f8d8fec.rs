use vstd::prelude::*;

verus! {

pub open spec fn overlaps(a: Interval, b: Interval) -> bool { a.lo <= b.hi && b.lo <= a.hi }


pub struct Interval { pub lo: nat, pub hi: nat }

pub enum IntervalTree {
    E,
    T { interval: Interval, max: nat, left: Box<IntervalTree>, right: Box<IntervalTree> },
}

pub open spec fn search_overlap(t: IntervalTree, q: Interval) -> bool decreases t {
    match t {
        IntervalTree::E => false,
        IntervalTree::T { interval, left, right, .. } =>
            overlaps(interval, q) || search_overlap(*left, q) || search_overlap(*right, q)
    }
}


pub proof fn no_overlap_empty(q: Interval) ensures !search_overlap(IntervalTree::E, q) {}

} // verus!