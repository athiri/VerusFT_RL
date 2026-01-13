use vstd::prelude::*;

verus! {

pub open spec fn ring_add_int(a: int, b: int) -> int {
    a + b
}


pub proof fn ring_add_assoc_int(a: int, b: int, c: int)
    ensures ring_add_int(ring_add_int(a, b), c) == ring_add_int(a, ring_add_int(b, c))
{
    assert((a + b) + c == a + (b + c));
}

} // verus!