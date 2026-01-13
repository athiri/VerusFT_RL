use vstd::prelude::*;

verus! {

pub open spec fn is_failure(r: TestResult) -> bool {
    match r {
        TestResult::Failure { .. } => true,
        _ => false,
    }
}

pub open spec fn is_success(r: TestResult) -> bool {
    r == TestResult::Success
}

pub open spec fn is_discarded(r: TestResult) -> bool {
    r == TestResult::Discarded
}


pub struct TestStats {
    pub successful: nat,
    pub failed: nat,
    pub discarded: nat,
    pub total: nat,
}

pub enum TestResult {
    Success,
    Failure { counterexample: nat },
    Discarded,
    GaveUp,
}

pub open spec fn add_result(stats: TestStats, r: TestResult) -> TestStats {
    TestStats {
        successful: stats.successful + if is_success(r) { 1nat } else { 0nat },
        failed: stats.failed + if is_failure(r) { 1nat } else { 0nat },
        discarded: stats.discarded + if is_discarded(r) { 1nat } else { 0nat },
        total: stats.total + 1,
    }
}


pub proof fn stats_consistency(stats: TestStats, r: TestResult)
    ensures add_result(stats, r).total == stats.total + 1
{
}

} // verus!