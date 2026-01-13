use vstd::prelude::*;

verus! {

pub open spec fn min_bound_bool() -> bool {
    false
}

pub open spec fn max_bound_bool() -> bool {
    true
}

pub open spec fn bool_le(a: bool, b: bool) -> bool {
    !a || b  // false <= anything, true <= true only
}


pub proof fn bounded_law_bool()
    ensures bool_le(min_bound_bool(), max_bound_bool())
{
    assert(!false || true);
}

} // verus!