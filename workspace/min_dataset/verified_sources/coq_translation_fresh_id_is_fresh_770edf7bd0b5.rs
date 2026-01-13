use vstd::prelude::*;

verus! {

pub type Id = nat;

pub open spec fn fresh_id(bound: Id) -> Id {
    bound + 1
}

pub open spec fn is_fresh(id: Id, bound: Id) -> bool {
    id > bound
}


pub proof fn fresh_id_is_fresh(bound: Id)
    ensures is_fresh(fresh_id(bound), bound)
{
    assert(fresh_id(bound) > bound);
}

} // verus!