use vstd::prelude::*;

verus! {

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

pub open spec fn ring_mul_int(a: int, b: int) -> int {
    a * b
}


pub proof fn ring_power_one_int(base: int)
    ensures ring_power_int(base, 1) == base
{
    // ring_power_int(base, 1) = ring_mul_int(base, ring_power_int(base, 0))
    //                        = ring_mul_int(base, 1)
    //                        = base
    assert(ring_power_int(base, 0) == 1);
    assert(ring_power_int(base, 1) == ring_mul_int(base, ring_power_int(base, 0)));
    assert(ring_mul_int(base, 1) == base);
}

} // verus!