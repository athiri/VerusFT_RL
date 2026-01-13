use vstd::prelude::*;

verus! {

pub open spec fn seq_range(n: nat) -> Seq<nat> {
    Seq::new(n, |i: int| i as nat)
}


pub proof fn range_elements(n: nat, i: nat)
    requires i < n
    ensures seq_range(n)[i as int] == i
{
}

} // verus!