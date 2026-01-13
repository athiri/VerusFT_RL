use vstd::prelude::*;

verus! {

pub open spec fn relax(dist: Seq<Option<nat>>, u: nat, v: nat, w: nat) -> Seq<Option<nat>>
{
    if u < dist.len() && v < dist.len() {
        match (dist[u as int], dist[v as int]) {
            (Some(du), Some(dv)) => {
                if du + w < dv {
                    dist.update(v as int, Some((du + w) as nat))
                } else {
                    dist
                }
            }
            (Some(du), None) => dist.update(v as int, Some((du + w) as nat)),
            _ => dist,
        }
    } else {
        dist
    }
}

pub proof fn relax_improves(dist: Seq<Option<nat>>, u: nat, v: nat, w: nat)
    requires u < dist.len(), v < dist.len()
    ensures match (relax(dist, u, v, w)[v as int], dist[v as int]) {
        (Some(new_d), Some(old_d)) => new_d <= old_d,
        (Some(_), None) => true,
        (None, _) => true,
    }
{
    match (dist[u as int], dist[v as int]) {
        (Some(du), Some(dv)) => {
            if du + w < dv {
            } else {
            }
        }
        (Some(du), None) => {
        }
        _ => {
        }
    }
}

} // verus!
