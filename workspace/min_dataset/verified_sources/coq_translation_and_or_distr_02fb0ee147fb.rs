use vstd::prelude::*;

verus! {

pub open spec fn bool_and(a: bool, b: bool) -> bool { a && b }

pub open spec fn bool_or(a: bool, b: bool) -> bool { a || b }


pub proof fn and_or_distr(a: bool, b: bool, c: bool)
    ensures bool_and(a, bool_or(b, c)) == bool_or(bool_and(a, b), bool_and(a, c))
{}

} // verus!