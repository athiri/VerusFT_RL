use vstd::prelude::*;

verus! {

pub type Id = nat;

pub open spec fn fresh_id(bound: Id) -> Id {
    bound + 1
}


pub proof fn fresh_ids_different(bound1: Id, bound2: Id)
    requires bound1 != bound2
    ensures fresh_id(bound1) != fresh_id(bound2)
{
}

} // verus!