use vstd::prelude::*;

verus! {

pub struct Edge { pub from: nat, pub to: nat, pub weight: int }


pub open spec fn relax_all(dist: Seq<Option<int>>, edges: Seq<Edge>, i: nat) -> Seq<Option<int>>
    decreases edges.len() - i
{
    if i >= edges.len() { dist }
    else {
        let e = edges[i as int];
        let new_dist = match dist[e.from as int] {
            None => dist,
            Some(du) => match dist[e.to as int] {
                None => dist.update(e.to as int, Some(du + e.weight)),
                Some(dv) => if du + e.weight < dv { dist.update(e.to as int, Some(du + e.weight)) } else { dist }
            }
        };
        relax_all(new_dist, edges, i + 1)
    }
}

} // verus!