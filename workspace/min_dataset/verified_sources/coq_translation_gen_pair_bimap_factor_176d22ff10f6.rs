use vstd::prelude::*;

verus! {

pub open spec fn gen_pair_bimap<A, B, C, D>(
    outputs: Set<(A, B)>,
    f: spec_fn(A) -> C,
    g: spec_fn(B) -> D
) -> Set<(C, D)> {
    Set::new(|p: (C, D)|
        exists|ab: (A, B)| outputs.contains(ab) && f(ab.0) == p.0 && g(ab.1) == p.1
    )
}


pub proof fn gen_pair_bimap_factor<A, B, C, D>(
    outputs: Set<(A, B)>,
    f: spec_fn(A) -> C,
    g: spec_fn(B) -> D,
    a: A,
    b: B
)
    requires outputs.contains((a, b))
    ensures gen_pair_bimap(outputs, f, g).contains((f(a), g(b)))
{
}

} // verus!