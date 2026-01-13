use vstd::prelude::*;

verus! {

pub open spec fn bool_or_identity() -> bool { false }

pub open spec fn bool_or(a: bool, b: bool) -> bool { a || b }


pub open spec fn mconcat_bool_or(s: Seq<bool>) -> bool
    decreases s.len()
{
    if s.len() == 0 { bool_or_identity() }
    else { bool_or(s[0], mconcat_bool_or(s.skip(1))) }
}

} // verus!