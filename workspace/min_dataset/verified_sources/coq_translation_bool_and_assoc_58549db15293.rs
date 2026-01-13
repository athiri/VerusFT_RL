use vstd::prelude::*;

verus! {

pub open spec fn bool_and(a: bool, b: bool) -> bool { a && b }


pub proof fn bool_and_assoc(a: bool, b: bool, c: bool)
    ensures bool_and(bool_and(a, b), c) == bool_and(a, bool_and(b, c))
{
}

} // verus!