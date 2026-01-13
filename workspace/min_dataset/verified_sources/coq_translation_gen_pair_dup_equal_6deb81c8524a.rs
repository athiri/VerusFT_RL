use vstd::prelude::*;

verus! {

pub open spec fn gen_pair_dup_outputs<T>(out: Set<T>) -> Set<(T, T)> {
    Set::new(|p: (T, T)| out.contains(p.0) && p.0 == p.1)
}


pub proof fn gen_pair_dup_equal<T>(out: Set<T>, p: (T, T))
    requires gen_pair_dup_outputs(out).contains(p)
    ensures p.0 == p.1
{
}

} // verus!