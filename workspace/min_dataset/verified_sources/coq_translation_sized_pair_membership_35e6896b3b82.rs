use vstd::prelude::*;

verus! {

pub open spec fn sized_pair<A, B>(
    gen_a: spec_fn(nat) -> Set<A>,
    gen_b: spec_fn(nat) -> Set<B>,
    size: nat
) -> Set<(A, B)> {
    Set::new(|p: (A, B)|
        gen_a(size).contains(p.0) && gen_b(size).contains(p.1)
    )
}


pub proof fn sized_pair_membership<A, B>(
    gen_a: spec_fn(nat) -> Set<A>,
    gen_b: spec_fn(nat) -> Set<B>,
    size: nat,
    a: A,
    b: B
)
    requires gen_a(size).contains(a), gen_b(size).contains(b)
    ensures sized_pair(gen_a, gen_b, size).contains((a, b))
{
}

} // verus!