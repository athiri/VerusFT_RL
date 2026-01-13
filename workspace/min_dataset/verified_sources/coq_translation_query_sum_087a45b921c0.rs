use vstd::prelude::*;

verus! {

pub enum SegTree { Leaf(nat), Node { sum: nat, left: Box<SegTree>, right: Box<SegTree> } }

pub open spec fn seg_sum(t: SegTree) -> nat {
    match t { SegTree::Leaf(v) => v, SegTree::Node { sum, .. } => sum }
}


pub open spec fn query_sum(t: SegTree, lo: nat, hi: nat, qlo: nat, qhi: nat) -> nat decreases t {
    if qlo >= hi || qhi <= lo { 0 }
    else if qlo <= lo && hi <= qhi { seg_sum(t) }
    else { match t {
        SegTree::Leaf(v) => v,
        SegTree::Node { left, right, .. } => {
            let mid = (lo + (hi - lo) / 2) as nat;
            query_sum(*left, lo, mid, qlo, qhi) + query_sum(*right, mid, hi, qlo, qhi)
        }
    }}
}

} // verus!