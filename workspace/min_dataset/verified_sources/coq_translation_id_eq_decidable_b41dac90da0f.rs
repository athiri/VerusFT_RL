use vstd::prelude::*;

verus! {

pub type Id = nat;

pub open spec fn id_eq(x: Id, y: Id) -> bool {
    x == y
}


pub proof fn id_eq_decidable(x: Id, y: Id)
    ensures id_eq(x, y) || !id_eq(x, y)
{
    // Trivially true by classical logic
}

} // verus!