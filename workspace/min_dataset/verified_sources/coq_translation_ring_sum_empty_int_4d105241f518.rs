use vstd::prelude::*;

verus! {

pub open spec fn ring_add_int(a: int, b: int) -> int {
    a + b
}


pub open spec fn ring_zero_int() -> int {
    0
}

pub open spec fn ring_sum_int(xs: Seq<int>) -> int
    decreases xs.len()
{
    if xs.len() == 0 {
        ring_zero_int()
    } else {
        ring_add_int(xs[0], ring_sum_int(xs.skip(1)))
    }
}


pub proof fn ring_sum_empty_int()
    ensures ring_sum_int(Seq::empty()) == ring_zero_int()
{
    // Trivially true
}

} // verus!