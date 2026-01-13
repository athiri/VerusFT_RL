use vstd::prelude::*;

verus! {

pub struct UnionFind {
    pub parent: Seq<nat>,
    pub rank: Seq<nat>,
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

} // verus!