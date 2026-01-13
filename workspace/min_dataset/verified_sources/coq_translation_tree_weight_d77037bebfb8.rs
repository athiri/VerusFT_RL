use vstd::prelude::*;

verus! {

pub struct WeightedEdge { pub u: nat, pub v: nat, pub weight: nat }

pub struct Graph { pub n: nat, pub edges: Seq<WeightedEdge> }


pub open spec fn tree_weight(g: Graph, tree_edges: Seq<nat>) -> nat decreases tree_edges.len() {
    if tree_edges.len() == 0 { 0 }
    else { g.edges[tree_edges[0] as int].weight + tree_weight(g, tree_edges.skip(1)) }
}

} // verus!