use vstd::prelude::*;

verus! {

pub open spec fn bool_and(a: bool, b: bool) -> bool { a && b }

pub open spec fn bool_and_identity() -> bool { true }


pub open spec fn mconcat_bool_and(s: Seq<bool>) -> bool
    decreases s.len()
{
    if s.len() == 0 { bool_and_identity() }
    else { bool_and(s[0], mconcat_bool_and(s.skip(1))) }
}

} // verus!