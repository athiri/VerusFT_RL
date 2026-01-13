use vstd::prelude::*;

verus! {

pub struct UnionFind {
    pub parent: Seq<nat>,
    pub rank: Seq<nat>,
}

pub open spec fn uf_init(n: nat) -> UnionFind {
    UnionFind {
        parent: Seq::new(n, |i: int| i as nat),
        rank: Seq::new(n, |_i: int| 0nat),
    }
}

pub open spec fn is_root(uf: UnionFind, x: nat) -> bool
    recommends x < uf.parent.len()
{
    uf.parent[x as int] == x
}


pub proof fn init_singletons(n: nat, x: nat)
    requires x < n
    ensures is_root(uf_init(n), x)
{
}

} // verus!