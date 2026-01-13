use vstd::prelude::*;

verus! {

pub open spec fn in_range(n: nat, lo: nat, hi: nat) -> bool {
    lo <= n && n < hi
}


pub open spec fn choose_outputs(lo: nat, hi: nat) -> Set<nat> {
    Set::new(|n: nat| in_range(n, lo, hi))
}


pub proof fn choose_singleton(n: nat)
    ensures
        choose_outputs(n, n + 1).contains(n),
        forall|m: nat| #[trigger] choose_outputs(n, n + 1).contains(m) ==> m == n,
{
}

} // verus!