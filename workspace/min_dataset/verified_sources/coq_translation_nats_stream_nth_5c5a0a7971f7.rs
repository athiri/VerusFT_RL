use vstd::prelude::*;

verus! {

pub open spec fn nats_stream() -> spec_fn(nat) -> nat { |n: nat| n }

pub open spec fn stream_nth(gen: spec_fn(nat) -> nat, n: nat) -> nat { gen(n) }


pub proof fn nats_stream_nth(n: nat) ensures stream_nth(nats_stream(), n) == n {}

} // verus!