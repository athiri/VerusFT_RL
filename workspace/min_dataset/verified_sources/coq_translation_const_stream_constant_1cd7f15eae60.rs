use vstd::prelude::*;

verus! {

pub open spec fn const_stream(x: nat) -> spec_fn(nat) -> nat { |_n: nat| x }

pub open spec fn stream_nth(gen: spec_fn(nat) -> nat, n: nat) -> nat { gen(n) }


pub proof fn const_stream_constant(x: nat, n: nat) ensures stream_nth(const_stream(x), n) == x {}

} // verus!