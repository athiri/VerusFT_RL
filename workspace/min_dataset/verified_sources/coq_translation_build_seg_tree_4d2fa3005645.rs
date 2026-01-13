use vstd::prelude::*;

verus! {

pub enum SegTree { Leaf(nat), Node { sum: nat, left: Box<SegTree>, right: Box<SegTree> } }

pub open spec fn seg_sum(t: SegTree) -> nat {
    match t { SegTree::Leaf(v) => v, SegTree::Node { sum, .. } => sum }
}


pub open spec fn build_seg_tree(s: Seq<nat>, lo: nat, hi: nat) -> SegTree decreases hi - lo {
    if lo + 1 >= hi { SegTree::Leaf(if lo < s.len() { s[lo as int] } else { 0 }) }
    else {
        let mid = (lo + (hi - lo) / 2) as nat;
        let left = build_seg_tree(s, lo, mid);
        let right = build_seg_tree(s, mid, hi);
        SegTree::Node { sum: seg_sum(left) + seg_sum(right), left: Box::new(left), right: Box::new(right) }
    }
}

} // verus!