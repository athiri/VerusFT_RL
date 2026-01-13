use vstd::prelude::*;

verus! {

pub open spec fn iterate(f: spec_fn(nat) -> nat, start: nat, n: nat) -> Seq<nat> decreases n {
    if n == 0 { Seq::empty() } else { seq![start] + iterate(f, f(start), (n-1) as nat) }
}

} // verus!