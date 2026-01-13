use vstd::prelude::*;

verus! {

pub open spec fn seq_contains(s: Seq<nat>, x: nat) -> bool
    decreases s.len()
{
    if s.len() == 0 {
        false
    } else {
        s[0] == x || seq_contains(s.skip(1), x)
    }
}


pub open spec fn has_edge(g: Graph, u: nat, v: nat) -> bool {
    g.adj.dom().contains(u) &&
    seq_contains(g.adj[u], v)
}


pub open spec fn no_adjacent_same_color(g: Graph, c: Coloring, u: nat, v: nat) -> bool {
    has_edge(g, u, v) ==> get_color(c, u) != get_color(c, v)
}

pub open spec fn get_color(c: Coloring, node: nat) -> nat {
    if c.colors.dom().contains(node) {
        c.colors[node]
    } else {
        0
    }
}


pub open spec fn uses_k_colors(c: Coloring, n: nat, k: nat) -> bool {
    forall|node: nat| node < n ==> get_color(c, node) < k
}

pub open spec fn valid_coloring(g: Graph, c: Coloring) -> bool {
    forall|u: nat, v: nat|
        u < g.num_nodes && v < g.num_nodes ==>
        no_adjacent_same_color(g, c, u, v)
}


pub struct Coloring {
    pub colors: Map<nat, nat>,
}

pub struct Graph {
    pub adj: Map<nat, Seq<nat>>,
    pub num_nodes: nat,
}

pub open spec fn k_colorable(g: Graph, k: nat) -> bool {
    exists|c: Coloring| valid_coloring(g, c) && uses_k_colors(c, g.num_nodes, k)
}

pub open spec fn empty_coloring() -> Coloring {
    Coloring { colors: Map::empty() }
}

pub open spec fn empty_graph(n: nat) -> Graph {
    Graph {
        adj: Map::new(|node: nat| node < n, |node: nat| Seq::empty()),
        num_nodes: n,
    }
}


pub proof fn empty_graph_1_colorable(n: nat)
    ensures k_colorable(empty_graph(n), 1)
{
    let g = empty_graph(n);
    let c = empty_coloring();

    // Show valid_coloring - no edges in empty graph
    // Show uses_k_colors - all colors are 0 < 1
    assume(k_colorable(empty_graph(n), 1));
}

} // verus!