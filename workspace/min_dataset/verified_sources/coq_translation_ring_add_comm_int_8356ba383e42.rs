use vstd::prelude::*;

verus! {

pub open spec fn ring_add_int(a: int, b: int) -> int {
    a + b
}


pub proof fn ring_add_comm_int(a: int, b: int)
    ensures ring_add_int(a, b) == ring_add_int(b, a)
{
    assert(a + b == b + a);
}

} // verus!