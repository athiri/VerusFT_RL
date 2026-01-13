use vstd::prelude::*;

verus! {

pub open spec fn ring_mul_int(a: int, b: int) -> int {
    a * b
}


pub proof fn ring_mul_comm_int(a: int, b: int)
    ensures ring_mul_int(a, b) == ring_mul_int(b, a)
{
    assert(a * b == b * a);
}

} // verus!