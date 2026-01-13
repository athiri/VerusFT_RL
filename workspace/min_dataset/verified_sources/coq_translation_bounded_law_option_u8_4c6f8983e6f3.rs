use vstd::prelude::*;

verus! {

pub open spec fn max_bound_u8() -> nat {
    255
}


pub open spec fn option_nat_le(a: Option<nat>, b: Option<nat>) -> bool {
    match (a, b) {
        (Option::None, _) => true,
        (Option::Some(_), Option::None) => false,
        (Option::Some(x), Option::Some(y)) => x <= y,
    }
}

pub open spec fn max_bound_option_u8() -> Option<nat> {
    Option::Some(max_bound_u8())
}

pub open spec fn min_bound_option_u8() -> Option<nat> {
    Option::None
}


pub proof fn bounded_law_option_u8()
    ensures option_nat_le(min_bound_option_u8(), max_bound_option_u8())
{
    assert(option_nat_le(Option::None, Option::Some(255)));
}

} // verus!