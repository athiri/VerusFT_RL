use vstd::prelude::*;

verus! {

pub open spec fn gen_triple_outputs<A, B, C>(
    out_a: Set<A>,
    out_b: Set<B>,
    out_c: Set<C>
) -> Set<(A, B, C)> {
    Set::new(|t: (A, B, C)|
        out_a.contains(t.0) && out_b.contains(t.1) && out_c.contains(t.2)
    )
}


pub proof fn gen_triple_membership<A, B, C>(
    out_a: Set<A>,
    out_b: Set<B>,
    out_c: Set<C>,
    a: A,
    b: B,
    c: C
)
    requires out_a.contains(a), out_b.contains(b), out_c.contains(c)
    ensures gen_triple_outputs(out_a, out_b, out_c).contains((a, b, c))
{
}

} // verus!