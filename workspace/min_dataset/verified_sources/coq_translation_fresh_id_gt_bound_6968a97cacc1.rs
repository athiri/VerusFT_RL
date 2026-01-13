use vstd::prelude::*;

verus! {

pub type Id = nat;

pub open spec fn fresh_id(bound: Id) -> Id {
    bound + 1
}


pub proof fn fresh_id_gt_bound(bound: Id)
    ensures fresh_id(bound) > bound
{
}

} // verus!