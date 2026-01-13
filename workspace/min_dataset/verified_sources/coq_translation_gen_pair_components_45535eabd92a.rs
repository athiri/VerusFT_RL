use vstd::prelude::*;

verus! {

pub open spec fn gen_pair_outputs<A, B>(
    out_a: Set<A>,
    out_b: Set<B>
) -> Set<(A, B)> {
    Set::new(|p: (A, B)| out_a.contains(p.0) && out_b.contains(p.1))
}


pub proof fn gen_pair_components<A, B>(out_a: Set<A>, out_b: Set<B>, p: (A, B))
    requires gen_pair_outputs(out_a, out_b).contains(p)
    ensures out_a.contains(p.0), out_b.contains(p.1)
{
}

} // verus!