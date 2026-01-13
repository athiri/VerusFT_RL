use vstd::prelude::*;

verus! {

pub open spec fn in_int_range(n: int, lo: int, hi: int) -> bool {
    lo <= n && n < hi
}


pub open spec fn choose_int_outputs(lo: int, hi: int) -> Set<int> {
    Set::new(|n: int| in_int_range(n, lo, hi))
}

pub open spec fn gen_int_filter(outputs: Set<int>, p: spec_fn(int) -> bool) -> Set<int> {
    Set::new(|n: int| outputs.contains(n) && p(n))
}


pub open spec fn gen_int_bound_outputs(bound: nat) -> Set<int> {
    choose_int_outputs(-(bound as int), (bound as int) + 1)
}

pub open spec fn gen_positive_outputs(bound: nat) -> Set<int> {
    gen_int_filter(gen_int_bound_outputs(bound), |n: int| n > 0)
}

pub proof fn gen_int_filter_restriction(outputs: Set<int>, p: spec_fn(int) -> bool, n: int)
    requires gen_int_filter(outputs, p).contains(n)
    ensures outputs.contains(n) && p(n)
{
}


pub proof fn gen_positive_correct(bound: nat, n: int)
    requires gen_positive_outputs(bound).contains(n)
    ensures n > 0
{
    gen_int_filter_restriction(gen_int_bound_outputs(bound), |m: int| m > 0, n);
}

} // verus!