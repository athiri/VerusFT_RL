use vstd::prelude::*;

verus! {

pub open spec fn in_range(n: nat, lo: nat, hi: nat) -> bool {
    lo <= n && n < hi
}


pub open spec fn choose_outputs(lo: nat, hi: nat) -> Set<nat> {
    Set::new(|n: nat| in_range(n, lo, hi))
}


pub proof fn choose_bounded(lo: nat, hi: nat, n: nat)
    requires choose_outputs(lo, hi).contains(n)
    ensures lo <= n && n < hi
{
}

} // verus!