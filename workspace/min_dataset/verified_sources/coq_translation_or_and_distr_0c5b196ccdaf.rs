use vstd::prelude::*;

verus! {

pub open spec fn bool_and(a: bool, b: bool) -> bool { a && b }

pub open spec fn bool_or(a: bool, b: bool) -> bool { a || b }


pub proof fn or_and_distr(a: bool, b: bool, c: bool)
    ensures bool_or(a, bool_and(b, c)) == bool_and(bool_or(a, b), bool_or(a, c))
{}

} // verus!