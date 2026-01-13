use vstd::prelude::*;

verus! {

pub struct Edge { pub src: nat, pub dst: nat }

pub struct Graph { pub edges: Seq<Edge>, pub n: nat }

pub open spec fn path(g: Graph, u: nat, v: nat, p: Seq<Edge>) -> bool
    decreases p.len()
{
    if p.len() == 0 {
        u == v
    } else {
        let e = p.first();
        e.src == u && g.edges.contains(e) && path(g, e.dst, v, p.drop_first())
    }
}

pub proof fn path_refl(g: Graph, u: nat)
    ensures path(g, u, u, Seq::empty())
{ reveal_with_fuel(path, 2); }

} // verus!
