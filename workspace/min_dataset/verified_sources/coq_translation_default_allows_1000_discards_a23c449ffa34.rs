use vstd::prelude::*;

verus! {

pub struct QCConfig {
    pub max_success: nat,
    pub max_discard_ratio: nat,
    pub max_size: nat,
    pub max_shrinks: nat,
    pub replay_seed: Option<nat>,
}

pub open spec fn default_config() -> QCConfig {
    QCConfig {
        max_success: 100,
        max_discard_ratio: 10,
        max_size: 100,
        max_shrinks: 1000,
        replay_seed: Option::None,
    }
}

pub open spec fn max_discards(config: QCConfig) -> nat {
    config.max_success * config.max_discard_ratio
}


pub proof fn default_allows_1000_discards()
    ensures max_discards(default_config()) == 1000
{
    assert(100 * 10 == 1000);
}

} // verus!