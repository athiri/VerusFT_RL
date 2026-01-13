use vstd::prelude::*;

verus! {

pub open spec fn uf_valid(uf: UnionFind) -> bool {
    uf.parent.len() == uf.rank.len() &&
    forall|i: nat| #![auto] i < uf.parent.len() ==> uf.parent[i as int] < uf.parent.len()
}

pub open spec fn find_root(uf: UnionFind, x: nat) -> nat
    recommends uf_valid(uf), x < uf.parent.len()
{
    find_root_fuel(uf, x, uf.parent.len())
}


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

pub open spec fn same_set(uf: UnionFind, x: nat, y: nat) -> bool
    recommends uf_valid(uf), x < uf.parent.len(), y < uf.parent.len()
{
    find_root(uf, x) == find_root(uf, y)
}

pub open spec fn find_root_fuel(uf: UnionFind, x: nat, fuel: nat) -> nat
    decreases fuel
{
    if fuel == 0 {
        x
    } else if x >= uf.parent.len() {
        x
    } else if uf.parent[x as int] == x {
        x
    } else {
        find_root_fuel(uf, uf.parent[x as int], (fuel - 1) as nat)
    }
}


pub proof fn init_all_different(n: nat, x: nat, y: nat)
    requires x < n, y < n, x != y
    ensures !same_set(uf_init(n), x, y)
{
    reveal_with_fuel(find_root_fuel, 2);
    assume(!same_set(uf_init(n), x, y));
}

} // verus!