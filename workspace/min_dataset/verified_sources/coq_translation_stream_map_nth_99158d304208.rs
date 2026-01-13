use vstd::prelude::*;

verus! {

pub open spec fn stream_map(f: spec_fn(nat) -> nat, gen: spec_fn(nat) -> nat) -> spec_fn(nat) -> nat { |n: nat| f(gen(n)) }

pub open spec fn stream_nth(gen: spec_fn(nat) -> nat, n: nat) -> nat { gen(n) }


pub proof fn stream_map_nth(f: spec_fn(nat) -> nat, g: spec_fn(nat) -> nat, n: nat)
    ensures stream_nth(stream_map(f, g), n) == f(g(n))
{}

} // verus!