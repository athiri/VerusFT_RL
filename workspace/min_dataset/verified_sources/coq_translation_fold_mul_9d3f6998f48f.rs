use vstd::prelude::*;

verus! {

pub open spec fn fold_mul(s: Seq<nat>) -> nat decreases s.len() {
    if s.len() == 0 { 1 } else { s[0] * fold_mul(s.skip(1)) }
}

} // verus!