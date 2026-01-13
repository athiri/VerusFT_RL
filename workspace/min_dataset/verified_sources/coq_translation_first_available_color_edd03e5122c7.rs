use vstd::prelude::*;

verus! {

pub open spec fn get_color(c: Coloring, node: nat) -> nat {
    if c.colors.dom().contains(node) {
        c.colors[node]
    } else {
        0
    }
}


pub struct Coloring {
    pub colors: Map<nat, nat>,
}

pub open spec fn neighbor_has_color(c: Coloring, neighbors: Seq<nat>, color: nat) -> bool
    decreases neighbors.len()
{
    if neighbors.len() == 0 {
        false
    } else {
        get_color(c, neighbors[0]) == color || neighbor_has_color(c, neighbors.skip(1), color)
    }
}

pub struct Graph {
    pub adj: Map<nat, Seq<nat>>,
    pub num_nodes: nat,
}


pub open spec fn first_available_color(g: Graph, c: Coloring, node: nat, color: nat) -> nat
    decreases g.num_nodes - color
{
    if color >= g.num_nodes {
        color
    } else {
        let neighbors = if g.adj.dom().contains(node) { g.adj[node] } else { Seq::empty() };
        if !neighbor_has_color(c, neighbors, color) {
            color
        } else {
            first_available_color(g, c, node, color + 1)
        }
    }
}

} // verus!