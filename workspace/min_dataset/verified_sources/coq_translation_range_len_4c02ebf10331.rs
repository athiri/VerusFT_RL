use vstd::prelude::*;

verus! {

pub open spec fn seq_range(n: nat) -> Seq<nat> {
    Seq::new(n, |i: int| i as nat)
}


pub proof fn range_len(n: nat)
    ensures seq_range(n).len() == n
{
}

} // verus!