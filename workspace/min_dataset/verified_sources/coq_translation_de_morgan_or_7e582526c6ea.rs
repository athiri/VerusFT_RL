use vstd::prelude::*;

verus! {

pub open spec fn bool_not(a: bool) -> bool { !a }

pub open spec fn bool_and(a: bool, b: bool) -> bool { a && b }

pub open spec fn bool_or(a: bool, b: bool) -> bool { a || b }


pub proof fn de_morgan_or(a: bool, b: bool)
    ensures bool_not(bool_or(a, b)) == bool_and(bool_not(a), bool_not(b))
{}

} // verus!