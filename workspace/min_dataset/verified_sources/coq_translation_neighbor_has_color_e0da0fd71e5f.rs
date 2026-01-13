use vstd::prelude::*;

verus! {

pub struct Coloring {
    pub colors: Map<nat, nat>,
}

pub open spec fn get_color(c: Coloring, node: nat) -> nat {
    if c.colors.dom().contains(node) {
        c.colors[node]
    } else {
        0
    }
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

} // verus!