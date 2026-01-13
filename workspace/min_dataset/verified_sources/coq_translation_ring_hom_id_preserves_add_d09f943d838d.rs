use vstd::prelude::*;

verus! {

pub open spec fn ring_hom_id(x: int) -> int {
    x
}

pub open spec fn ring_add_int(a: int, b: int) -> int {
    a + b
}


pub proof fn ring_hom_id_preserves_add(a: int, b: int)
    ensures ring_hom_id(ring_add_int(a, b)) == ring_add_int(ring_hom_id(a), ring_hom_id(b))
{
    // Trivially true
}

} // verus!