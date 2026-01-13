use vstd::prelude::*;

verus! {

pub open spec fn ring_neg_int(a: int) -> int {
    -a
}

pub open spec fn ring_add_int(a: int, b: int) -> int {
    a + b
}


pub proof fn ring_neg_distrib_int(a: int, b: int)
    ensures ring_neg_int(ring_add_int(a, b)) == ring_add_int(ring_neg_int(a), ring_neg_int(b))
{
    assert(-(a + b) == -a + -b);
}

} // verus!