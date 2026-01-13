use vstd::prelude::*;

verus! {

pub open spec fn ring_hom_id(x: int) -> int {
    x
}

pub open spec fn ring_one_int() -> int {
    1
}


pub proof fn ring_hom_id_preserves_one()
    ensures ring_hom_id(ring_one_int()) == ring_one_int()
{
    // Trivially true
}

} // verus!