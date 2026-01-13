use vstd::prelude::*;

verus! {

pub open spec fn in_range(n: nat, lo: nat, hi: nat) -> bool {
    lo <= n && n < hi
}


pub open spec fn choose_outputs(lo: nat, hi: nat) -> Set<nat> {
    Set::new(|n: nat| in_range(n, lo, hi))
}


pub proof fn choose_nonempty(lo: nat, hi: nat)
    requires lo < hi
    ensures choose_outputs(lo, hi).contains(lo)
{
}

} // verus!