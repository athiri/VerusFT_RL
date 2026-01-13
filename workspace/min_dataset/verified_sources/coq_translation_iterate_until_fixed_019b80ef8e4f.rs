use vstd::prelude::*;

verus! {

pub open spec fn iterate_until_fixed(f: spec_fn(nat) -> nat, x: nat, fuel: nat) -> nat decreases fuel {
    if fuel == 0 { x }
    else if f(x) == x { x }
    else { iterate_until_fixed(f, f(x), (fuel - 1) as nat) }
}

} // verus!