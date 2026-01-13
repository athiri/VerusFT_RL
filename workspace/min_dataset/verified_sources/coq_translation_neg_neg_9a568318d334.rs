use vstd::prelude::*;

verus! {

pub proof fn neg_neg(a: int) ensures -(-a) == a {}

} // verus!