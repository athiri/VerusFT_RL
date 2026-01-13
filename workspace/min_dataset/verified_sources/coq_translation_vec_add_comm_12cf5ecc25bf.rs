use vstd::prelude::*;

verus! {

pub open spec fn vec_add(v1: Seq<nat>, v2: Seq<nat>) -> Seq<nat>
    recommends v1.len() == v2.len()
{ Seq::new(v1.len(), |i: int| v1[i] + v2[i]) }


pub proof fn vec_add_comm(v1: Seq<nat>, v2: Seq<nat>)
    requires v1.len() == v2.len()
    ensures vec_add(v1, v2) =~= vec_add(v2, v1)
{}

} // verus!