use vstd::prelude::*;

verus! {

pub open spec fn gen_pair_map_fst<A, B, C>(
    outputs: Set<(A, B)>,
    f: spec_fn(A) -> C
) -> Set<(C, B)> {
    Set::new(|p: (C, B)|
        exists|ab: (A, B)| outputs.contains(ab) && f(ab.0) == p.0 && ab.1 == p.1
    )
}


pub proof fn gen_pair_map_fst_contains<A, B, C>(
    outputs: Set<(A, B)>,
    f: spec_fn(A) -> C,
    a: A,
    b: B,
    c: C
)
    requires outputs.contains((a, b)), c == f(a)
    ensures gen_pair_map_fst(outputs, f).contains((c, b))
{
}

} // verus!