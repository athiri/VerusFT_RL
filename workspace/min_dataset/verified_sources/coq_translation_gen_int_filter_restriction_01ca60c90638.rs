use vstd::prelude::*;

verus! {

pub open spec fn gen_int_filter(outputs: Set<int>, p: spec_fn(int) -> bool) -> Set<int> {
    Set::new(|n: int| outputs.contains(n) && p(n))
}


pub proof fn gen_int_filter_restriction(outputs: Set<int>, p: spec_fn(int) -> bool, n: int)
    requires gen_int_filter(outputs, p).contains(n)
    ensures outputs.contains(n) && p(n)
{
}

} // verus!