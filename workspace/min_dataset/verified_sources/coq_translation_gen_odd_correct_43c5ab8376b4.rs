use vstd::prelude::*;

verus! {

pub open spec fn in_range(n: nat, lo: nat, hi: nat) -> bool {
    lo <= n && n < hi
}


pub open spec fn gen_nat_filter(outputs: Set<nat>, p: spec_fn(nat) -> bool) -> Set<nat> {
    Set::new(|n: nat| outputs.contains(n) && p(n))
}

pub open spec fn choose_outputs(lo: nat, hi: nat) -> Set<nat> {
    Set::new(|n: nat| in_range(n, lo, hi))
}


pub proof fn gen_nat_filter_restriction(outputs: Set<nat>, p: spec_fn(nat) -> bool, n: nat)
    requires gen_nat_filter(outputs, p).contains(n)
    ensures outputs.contains(n) && p(n)
{
}

pub open spec fn gen_nat_bound_outputs(bound: nat) -> Set<nat> {
    choose_outputs(0, bound + 1)
}

pub open spec fn gen_odd_outputs(bound: nat) -> Set<nat> {
    gen_nat_filter(gen_nat_bound_outputs(bound), |n: nat| n % 2 == 1)
}


pub proof fn gen_odd_correct(bound: nat, n: nat)
    requires gen_odd_outputs(bound).contains(n)
    ensures n % 2 == 1 && n <= bound
{
    gen_nat_filter_restriction(gen_nat_bound_outputs(bound), |m: nat| m % 2 == 1, n);
}

} // verus!