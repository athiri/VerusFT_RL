use vstd::prelude::*;

verus! {

pub enum TestOutcome {
    Pass,
    Fail { counterexample: nat },
    Discard,
}

pub open spec fn conditional_outcome(precond: bool, result: bool, value: nat) -> TestOutcome {
    if !precond { TestOutcome::Discard }
    else if result { TestOutcome::Pass }
    else { TestOutcome::Fail { counterexample: value } }
}


pub proof fn precond_true_no_discard(result: bool, value: nat)
    ensures conditional_outcome(true, result, value) != TestOutcome::Discard
{
}

} // verus!