use vstd::prelude::*;

verus! {

pub open spec fn shrink_int_towards_zero(x: int) -> int {
    if x > 0 {
        x - 1
    } else if x < 0 {
        x + 1
    } else {
        0
    }
}


pub proof fn verify_shrink_int_towards_zero(x: int)
    requires x != 0
    ensures
        (x > 0 ==> shrink_int_towards_zero(x) >= 0 && shrink_int_towards_zero(x) < x),
        (x < 0 ==> shrink_int_towards_zero(x) <= 0 && shrink_int_towards_zero(x) > x)
{
}

} // verus!