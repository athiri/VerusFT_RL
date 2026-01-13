use vstd::prelude::*;

verus! {

pub open spec fn ring_neg_int(a: int) -> int {
    -a
}


pub proof fn ring_neg_neg_int(a: int)
    ensures ring_neg_int(ring_neg_int(a)) == a
{
    assert(-(-a) == a);
}

} // verus!