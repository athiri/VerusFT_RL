use vstd::prelude::*;

verus! {

pub open spec fn bool_or(a: bool, b: bool) -> bool { a || b }


pub proof fn bool_or_assoc(a: bool, b: bool, c: bool)
    ensures bool_or(bool_or(a, b), c) == bool_or(a, bool_or(b, c))
{
}

} // verus!