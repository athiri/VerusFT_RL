use vstd::prelude::*;

verus! {

pub enum BTree {
    Node { key: nat, children: Seq<Box<BTree>> },
}

pub open spec fn bt_link(t1: BTree, t2: BTree) -> BTree {
    match (t1, t2) {
        (BTree::Node { key: k1, children: c1 }, BTree::Node { key: k2, children: c2 }) =>
            if k1 <= k2 {
                BTree::Node { key: k1, children: c1.push(Box::new(t2)) }
            } else {
                BTree::Node { key: k2, children: c2.push(Box::new(t1)) }
            }
    }
}


pub open spec fn bq_merge_helper(
    t1: Seq<Option<BTree>>,
    t2: Seq<Option<BTree>>,
    carry: Option<BTree>,
    pos: nat
) -> Seq<Option<BTree>>
    decreases t1.len() + t2.len() - pos
{
    if pos >= t1.len() && pos >= t2.len() {
        match carry {
            None => Seq::empty(),
            Some(c) => seq![Some(c)],
        }
    } else {
        let tree1 = if pos < t1.len() { t1[pos as int] } else { None };
        let tree2 = if pos < t2.len() { t2[pos as int] } else { None };

        let (result, new_carry) = match (tree1, tree2, carry) {
            (None, None, None) => (None, None),
            (Some(a), None, None) => (Some(a), None),
            (None, Some(b), None) => (Some(b), None),
            (None, None, Some(c)) => (Some(c), None),
            (Some(a), Some(b), None) => (None, Some(bt_link(a, b))),
            (Some(a), None, Some(c)) => (None, Some(bt_link(a, c))),
            (None, Some(b), Some(c)) => (None, Some(bt_link(b, c))),
            (Some(a), Some(b), Some(c)) => (Some(c), Some(bt_link(a, b))),
        };

        seq![result] + bq_merge_helper(t1, t2, new_carry, pos + 1)
    }
}

} // verus!