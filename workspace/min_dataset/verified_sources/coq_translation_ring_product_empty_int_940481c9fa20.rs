use vstd::prelude::*;

verus! {

pub open spec fn ring_mul_int(a: int, b: int) -> int {
    a * b
}


pub open spec fn ring_product_int(xs: Seq<int>) -> int
    decreases xs.len()
{
    if xs.len() == 0 {
        ring_one_int()
    } else {
        ring_mul_int(xs[0], ring_product_int(xs.skip(1)))
    }
}

pub open spec fn ring_one_int() -> int {
    1
}


pub proof fn ring_product_empty_int()
    ensures ring_product_int(Seq::empty()) == ring_one_int()
{
    // Trivially true
}

} // verus!