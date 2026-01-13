use vstd::prelude::*;

verus! {

pub open spec fn gen_map<A, B>(
    gen_a: spec_fn(nat, nat) -> A,
    f: spec_fn(A) -> B,
    seed: nat,
    size: nat
) -> B {
    f(gen_a(seed, size))
}


pub proof fn gen_map_identity<A>(gen_a: spec_fn(nat, nat) -> A, seed: nat, size: nat)
    ensures gen_map(gen_a, |a: A| a, seed, size) == gen_a(seed, size)
{
    // Trivially true by definition
}

} // verus!