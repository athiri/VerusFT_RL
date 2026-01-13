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


pub open spec fn bq_insert_tree(t: BTree, trees: Seq<Option<BTree>>, pos: nat) -> Seq<Option<BTree>>
    decreases trees.len() - pos
{
    if pos >= trees.len() {
        trees.push(Some(t))
    } else {
        match trees[pos as int] {
            None => trees.update(pos as int, Some(t)),
            Some(existing) => {
                let linked = bt_link(t, existing);
                bq_insert_tree(linked, trees.update(pos as int, None), pos + 1)
            }
        }
    }
}

} // verus!