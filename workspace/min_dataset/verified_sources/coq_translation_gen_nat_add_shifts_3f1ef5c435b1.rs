use vstd::prelude::*;

verus! {

pub open spec fn gen_nat_map(outputs: Set<nat>, f: spec_fn(nat) -> nat) -> Set<nat> {
    Set::new(|n: nat| exists|m: nat| outputs.contains(m) && f(m) == n)
}


pub open spec fn in_range(n: nat, lo: nat, hi: nat) -> bool {
    lo <= n && n < hi
}

pub open spec fn gen_nat_add(outputs: Set<nat>, k: nat) -> Set<nat> {
    gen_nat_map(outputs, |n: nat| n + k)
}

pub open spec fn choose_outputs(lo: nat, hi: nat) -> Set<nat> {
    Set::new(|n: nat| in_range(n, lo, hi))
}


pub proof fn gen_nat_add_shifts(lo: nat, hi: nat, k: nat)
    ensures
        forall|n: nat| gen_nat_add(choose_outputs(lo, hi), k).contains(n) <==>
            in_range(n, lo + k, hi + k)
{
    assert forall|n: nat| gen_nat_add(choose_outputs(lo, hi), k).contains(n) <==>
        in_range(n, lo + k, hi + k) by {
        if gen_nat_add(choose_outputs(lo, hi), k).contains(n) {
            let m = choose|m: nat| choose_outputs(lo, hi).contains(m) && m + k == n;
            assert(in_range(m, lo, hi));
            assert(n == m + k);
        }
        if in_range(n, lo + k, hi + k) {
            let m = (n - k) as nat;
            assert(choose_outputs(lo, hi).contains(m));
        }
    }
}

} // verus!