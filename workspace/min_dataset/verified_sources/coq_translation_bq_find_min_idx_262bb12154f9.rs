use vstd::prelude::*;

verus! {

pub enum BTree {
    Node { key: nat, children: Seq<Box<BTree>> },
}

pub open spec fn bt_key(t: BTree) -> nat {
    match t { BTree::Node { key, .. } => key }
}


pub open spec fn bq_find_min_idx(trees: Seq<Option<BTree>>, best_idx: Option<nat>, pos: nat) -> Option<nat>
    decreases trees.len() - pos
{
    if pos >= trees.len() {
        best_idx
    } else {
        let new_best = match (trees[pos as int], best_idx) {
            (None, b) => b,
            (Some(t), None) => Some(pos),
            (Some(t), Some(bi)) => {
                match trees[bi as int] {
                    None => Some(pos),
                    Some(best_t) => if bt_key(t) < bt_key(best_t) { Some(pos) } else { Some(bi) },
                }
            }
        };
        bq_find_min_idx(trees, new_best, pos + 1)
    }
}

} // verus!