use vstd::prelude::*;

verus! {

pub open spec fn bool_not(a: bool) -> bool { !a }

pub open spec fn bool_and(a: bool, b: bool) -> bool { a && b }

pub open spec fn bool_or(a: bool, b: bool) -> bool { a || b }


pub proof fn de_morgan_and(a: bool, b: bool)
    ensures bool_not(bool_and(a, b)) == bool_or(bool_not(a), bool_not(b))
{}

} // verus!