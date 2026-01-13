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


pub proof fn precond_false_discards(result: bool, value: nat)
    ensures conditional_outcome(false, result, value) == TestOutcome::Discard
{
}

} // verus!