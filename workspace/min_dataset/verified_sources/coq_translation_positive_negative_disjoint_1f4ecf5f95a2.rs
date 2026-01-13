use vstd::prelude::*;

verus! {

pub open spec fn in_int_range(n: int, lo: int, hi: int) -> bool {
    lo <= n && n < hi
}


pub open spec fn choose_int_outputs(lo: int, hi: int) -> Set<int> {
    Set::new(|n: int| in_int_range(n, lo, hi))
}

pub proof fn gen_negative_correct(bound: nat, n: int)
    requires gen_negative_outputs(bound).contains(n)
    ensures n < 0
{
    gen_int_filter_restriction(gen_int_bound_outputs(bound), |m: int| m < 0, n);
}

pub proof fn gen_int_filter_restriction(outputs: Set<int>, p: spec_fn(int) -> bool, n: int)
    requires gen_int_filter(outputs, p).contains(n)
    ensures outputs.contains(n) && p(n)
{
}


pub open spec fn gen_int_bound_outputs(bound: nat) -> Set<int> {
    choose_int_outputs(-(bound as int), (bound as int) + 1)
}

pub open spec fn gen_int_filter(outputs: Set<int>, p: spec_fn(int) -> bool) -> Set<int> {
    Set::new(|n: int| outputs.contains(n) && p(n))
}

pub proof fn gen_positive_correct(bound: nat, n: int)
    requires gen_positive_outputs(bound).contains(n)
    ensures n > 0
{
    gen_int_filter_restriction(gen_int_bound_outputs(bound), |m: int| m > 0, n);
}


pub open spec fn gen_negative_outputs(bound: nat) -> Set<int> {
    gen_int_filter(gen_int_bound_outputs(bound), |n: int| n < 0)
}

pub open spec fn gen_int_intersect(out1: Set<int>, out2: Set<int>) -> Set<int> {
    out1.intersect(out2)
}

pub open spec fn gen_positive_outputs(bound: nat) -> Set<int> {
    gen_int_filter(gen_int_bound_outputs(bound), |n: int| n > 0)
}


pub proof fn positive_negative_disjoint(bound: nat)
    ensures gen_int_intersect(gen_positive_outputs(bound), gen_negative_outputs(bound)) =~= Set::empty()
{
    assert forall|n: int| !gen_int_intersect(gen_positive_outputs(bound), gen_negative_outputs(bound)).contains(n) by {
        if gen_positive_outputs(bound).contains(n) {
            gen_positive_correct(bound, n);
            assert(n > 0);
        }
        if gen_negative_outputs(bound).contains(n) {
            gen_negative_correct(bound, n);
            assert(n < 0);
        }
    }
}

} // verus!