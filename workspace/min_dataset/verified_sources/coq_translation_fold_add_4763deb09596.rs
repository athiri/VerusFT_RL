use vstd::prelude::*;

verus! {

pub open spec fn fold_add(s: Seq<nat>) -> nat decreases s.len() {
    if s.len() == 0 { 0 } else { s[0] + fold_add(s.skip(1)) }
}

} // verus!