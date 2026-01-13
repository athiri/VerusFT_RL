use vstd::prelude::*;

verus! {

pub open spec fn sum_weights(weights: Seq<nat>) -> nat
    decreases weights.len()
{
    if weights.len() == 0 {
        0
    } else {
        weights[0] + sum_weights(weights.drop_first())
    }
}

} // verus!