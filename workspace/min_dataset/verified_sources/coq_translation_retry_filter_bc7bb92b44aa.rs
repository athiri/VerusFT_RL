use vstd::prelude::*;

verus! {

pub open spec fn retry_filter(attempts: nat, max_attempts: nat, accepted: bool) -> bool
    decreases max_attempts - attempts
{
    if accepted {
        true
    } else if attempts >= max_attempts {
        false
    } else {
        // Would need more randomness to actually retry
        false
    }
}

} // verus!