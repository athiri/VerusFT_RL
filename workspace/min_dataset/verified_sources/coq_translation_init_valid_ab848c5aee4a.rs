use vstd::prelude::*;

verus! {

pub struct UnionFind {
    pub parent: Seq<nat>,
    pub rank: Seq<nat>,
}

pub open spec fn uf_valid(uf: UnionFind) -> bool {
    uf.parent.len() == uf.rank.len() &&
    forall|i: nat| #![auto] i < uf.parent.len() ==> uf.parent[i as int] < uf.parent.len()
}

pub open spec fn uf_init(n: nat) -> UnionFind {
    UnionFind {
        parent: Seq::new(n, |i: int| i as nat),
        rank: Seq::new(n, |_i: int| 0nat),
    }
}


pub proof fn init_valid(n: nat)
    ensures uf_valid(uf_init(n))
{
}

} // verus!