use vstd::prelude::*;

verus! {

pub open spec fn ring_mul_int(a: int, b: int) -> int {
    a * b
}

pub open spec fn ring_one_int() -> int {
    1
}


pub open spec fn ring_power_int(base: int, exp: nat) -> int
    decreases exp
{
    if exp == 0 {
        ring_one_int()
    } else {
        ring_mul_int(base, ring_power_int(base, (exp - 1) as nat))
    }
}

} // verus!