use vstd::prelude::*;

verus! {

pub struct RunnerConfig {
    pub max_success: nat,     // Tests needed for success
    pub max_discard: nat,     // Max discards before giving up
    pub max_shrinks: nat,     // Max shrinking attempts
}

pub struct TestStats {
    pub successful: nat,
    pub failed: nat,
    pub discarded: nat,
    pub total: nat,
}

pub open spec fn is_complete(config: RunnerConfig, stats: TestStats) -> bool {
    stats.successful >= config.max_success || stats.failed > 0 || stats.discarded >= config.max_discard
}

pub open spec fn should_continue(config: RunnerConfig, stats: TestStats) -> bool {
    stats.successful < config.max_success &&
    stats.failed == 0 &&
    stats.discarded < config.max_discard
}


pub proof fn continue_means_not_complete(config: RunnerConfig, stats: TestStats)
    requires should_continue(config, stats)
    ensures !is_complete(config, stats)
{
}

} // verus!