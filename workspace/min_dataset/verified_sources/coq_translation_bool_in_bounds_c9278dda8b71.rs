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


pub proof fn bool_in_bounds(b: bool)
    ensures bool_le(min_bound_bool(), b) && bool_le(b, max_bound_bool())
{
    assert(!false || b);  // false <= b
    assert(!b || true);   // b <= true
}

} // verus!