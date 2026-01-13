use vstd::prelude::*;

verus! {

pub struct BTree {
    pub key: nat,
    pub rank: nat,
}

pub open spec fn bt_key(t: BTree) -> nat {
    t.key
}


pub open spec fn bq_find_min_helper(trees: Seq<Option<BTree>>, best: Option<nat>) -> Option<nat>
    decreases trees.len()
{
    if trees.len() == 0 {
        best
    } else {
        let new_best = match (trees[0], best) {
            (None, b) => b,
            (Some(t), None) => Some(bt_key(t)),
            (Some(t), Some(b)) => if bt_key(t) < b { Some(bt_key(t)) } else { Some(b) },
        };
        bq_find_min_helper(trees.skip(1), new_best)
    }
}

} // verus!