use vstd::prelude::*;

verus! {

pub type Id = nat;

pub open spec fn id_lt(x: Id, y: Id) -> bool {
    x < y
}


pub proof fn id_lt_total(x: Id, y: Id)
    ensures id_lt(x, y) || x == y || id_lt(y, x)
{
    // Natural numbers have total ordering
}

} // verus!