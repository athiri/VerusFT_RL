use vstd::prelude::*;

verus! {

pub open spec fn in_range(n: nat, lo: nat, hi: nat) -> bool {
    lo <= n && n < hi
}


pub open spec fn choose_outputs(lo: nat, hi: nat) -> Set<nat> {
    Set::new(|n: nat| in_range(n, lo, hi))
}


pub proof fn choose_complete(lo: nat, hi: nat, n: nat)
    requires lo <= n && n < hi
    ensures choose_outputs(lo, hi).contains(n)
{
}

} // verus!