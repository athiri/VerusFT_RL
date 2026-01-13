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


pub proof fn gen_map_composition<A, B, C>(
    gen_a: spec_fn(nat, nat) -> A,
    f: spec_fn(A) -> B,
    g: spec_fn(B) -> C,
    seed: nat,
    size: nat
)
    ensures gen_map(gen_a, |a: A| g(f(a)), seed, size) ==
            gen_map(|s: nat, sz: nat| gen_map(gen_a, f, s, sz), g, seed, size)
{
    // LHS: g(f(gen_a(seed, size)))
    // RHS: g(gen_map(gen_a, f, seed, size)) = g(f(gen_a(seed, size)))
}

} // verus!