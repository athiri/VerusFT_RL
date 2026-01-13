use vstd::prelude::*;

verus! {

pub open spec fn seq_product_rec(s: Seq<nat>) -> nat
    decreases s.len()
{
    if s.len() == 0 {
        1
    } else {
        s[0] * seq_product_rec(s.skip(1))
    }
}

} // verus!