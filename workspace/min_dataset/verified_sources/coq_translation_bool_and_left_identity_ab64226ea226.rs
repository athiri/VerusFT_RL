use vstd::prelude::*;

verus! {

pub open spec fn bool_and(a: bool, b: bool) -> bool { a && b }

pub open spec fn bool_and_identity() -> bool { true }


pub proof fn bool_and_left_identity(x: bool)
    ensures bool_and(bool_and_identity(), x) == x
{
}

} // verus!