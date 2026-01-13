use vstd::prelude::*;

verus! {

pub struct ReplayInfo {
    pub seed: nat,
    pub size: nat,
    pub test_number: nat,
}

pub open spec fn replay_produces_same_test(info1: ReplayInfo, info2: ReplayInfo) -> bool {
    info1.seed == info2.seed && info1.size == info2.size ==>
        info1.test_number == info2.test_number
}


pub proof fn same_seed_same_replay(info: ReplayInfo)
    ensures replay_produces_same_test(info, info)
{
}

} // verus!