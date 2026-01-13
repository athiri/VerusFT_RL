use vstd::prelude::*;

verus! {

pub open spec fn dot_product(v1: Seq<nat>, v2: Seq<nat>) -> nat
    recommends v1.len() == v2.len()
    decreases v1.len()
{
    if v1.len() == 0 { 0 }
    else { v1[0] * v2[0] + dot_product(v1.skip(1), v2.skip(1)) }
}

} // verus!