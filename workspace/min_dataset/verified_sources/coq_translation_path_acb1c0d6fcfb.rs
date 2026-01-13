use vstd::prelude::*;

verus! {

pub struct Edge { pub src: nat, pub dst: nat }

pub struct Graph { pub edges: Seq<Edge>, pub n: nat }

pub open spec fn has_edge(g: Graph, u: nat, v: nat) -> bool {
    exists|i: int| #![auto] 0 <= i < g.edges.len() as int &&
        g.edges[i].src == u && g.edges[i].dst == v
}


pub open spec fn path(g: Graph, u: nat, v: nat, p: Seq<nat>) -> bool decreases p.len() {
    if p.len() == 0 { u == v }
    else { has_edge(g, u, p[0]) && path(g, p[0], v, p.skip(1)) }
}

} // verus!