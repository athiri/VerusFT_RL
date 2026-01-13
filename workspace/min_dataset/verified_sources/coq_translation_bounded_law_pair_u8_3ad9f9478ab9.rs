use vstd::prelude::*;

verus! {

pub open spec fn max_bound_u8() -> nat {
    255
}

pub open spec fn min_bound_u8() -> nat {
    0
}


pub open spec fn min_bound_pair_u8() -> (nat, nat) {
    (min_bound_u8(), min_bound_u8())
}

pub open spec fn pair_nat_le(a: (nat, nat), b: (nat, nat)) -> bool {
    a.0 <= b.0 && a.1 <= b.1
}

pub open spec fn max_bound_pair_u8() -> (nat, nat) {
    (max_bound_u8(), max_bound_u8())
}


pub proof fn bounded_law_pair_u8()
    ensures pair_nat_le(min_bound_pair_u8(), max_bound_pair_u8())
{
    assert(0 <= 255);
}

} // verus!