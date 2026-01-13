use vstd::prelude::*;

verus! {

pub open spec fn find_bucket(weights: Seq<nat>, value: nat, cumulative: nat) -> int
    decreases weights.len()
{
    if weights.len() == 0 {
        0
    } else if value < cumulative + weights[0] {
        0
    } else {
        1 + find_bucket(weights.drop_first(), value, cumulative + weights[0])
    }
}

} // verus!