use vstd::prelude::*;

verus! {

pub struct BTree {
    pub key: nat,
    pub rank: nat,
}

pub open spec fn bt_rank(t: BTree) -> nat {
    t.rank
}

pub open spec fn bt_link(t1: BTree, t2: BTree) -> BTree
    recommends bt_rank(t1) == bt_rank(t2)
{
    if t1.key <= t2.key {
        BTree { key: t1.key, rank: t1.rank + 1 }
    } else {
        BTree { key: t2.key, rank: t2.rank + 1 }
    }
}


pub proof fn link_increases_rank(t1: BTree, t2: BTree)
    requires bt_rank(t1) == bt_rank(t2)
    ensures bt_rank(bt_link(t1, t2)) == bt_rank(t1) + 1
{
}

} // verus!