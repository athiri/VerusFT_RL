use vstd::prelude::*;

verus! {

pub open spec fn in_range(n: nat, lo: nat, hi: nat) -> bool {
    lo <= n && n < hi
}


pub open spec fn choose_outputs(lo: nat, hi: nat) -> Set<nat> {
    Set::new(|n: nat| in_range(n, lo, hi))
}

pub open spec fn gen_outputs_bounded(outputs: Set<nat>, bound: nat) -> bool {
    forall|n: nat| outputs.contains(n) ==> n <= bound
}

pub proof fn choose_bounded(lo: nat, hi: nat, n: nat)
    requires choose_outputs(lo, hi).contains(n)
    ensures lo <= n && n < hi
{
}


pub proof fn choose_is_bounded(lo: nat, hi: nat)
    requires hi > 0
    ensures gen_outputs_bounded(choose_outputs(lo, hi), (hi - 1) as nat)
{
    assert forall|n: nat| choose_outputs(lo, hi).contains(n) implies n <= (hi - 1) as nat by {
        choose_bounded(lo, hi, n);
    }
}

} // verus!