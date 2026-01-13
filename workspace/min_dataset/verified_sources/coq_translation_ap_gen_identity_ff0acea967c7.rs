use vstd::prelude::*;

verus! {

pub open spec fn pure_gen<A>(a: A) -> spec_fn(nat, nat) -> A {
    |_seed: nat, _size: nat| a
}

pub open spec fn ap_gen<A, B>(
    gf: spec_fn(nat, nat) -> spec_fn(A) -> B,
    ga: spec_fn(nat, nat) -> A
) -> spec_fn(nat, nat) -> B {
    |seed: nat, size: nat| gf(seed, size)(ga(seed + 1, size))
}


pub proof fn ap_gen_identity<A>(ga: spec_fn(nat, nat) -> A, seed: nat, size: nat)
    ensures ap_gen(pure_gen(|a: A| a), ga)(seed, size) == ga(seed + 1, size)
{
    let gf = pure_gen(|a: A| a);
    assert(gf(seed, size) == (|a: A| a));
    assert(ap_gen(gf, ga)(seed, size) == gf(seed, size)(ga(seed + 1, size)));
}

} // verus!