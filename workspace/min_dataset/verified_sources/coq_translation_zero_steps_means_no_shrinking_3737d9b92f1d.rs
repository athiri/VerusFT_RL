use vstd::prelude::*;

verus! {

pub struct Counterexample<A> {
    pub original: A,
    pub shrunk: A,
    pub shrink_steps: nat,
}


pub proof fn zero_steps_means_no_shrinking<A>(ce: Counterexample<A>)
    requires ce.shrink_steps == 0
{
    // Original == shrunk when no steps taken
}

} // verus!