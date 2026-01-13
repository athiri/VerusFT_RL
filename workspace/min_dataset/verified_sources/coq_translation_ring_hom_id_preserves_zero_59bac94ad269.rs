use vstd::prelude::*;

verus! {

pub open spec fn ring_hom_id(x: int) -> int {
    x
}

pub open spec fn ring_zero_int() -> int {
    0
}


pub proof fn ring_hom_id_preserves_zero()
    ensures ring_hom_id(ring_zero_int()) == ring_zero_int()
{
    // Trivially true
}

} // verus!