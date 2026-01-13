use vstd::prelude::*;

verus! {

pub type Label = nat;

pub struct Classification {
    pub label: Label,
    pub count: nat,
}

pub type ClassificationMap = Seq<Classification>;

pub open spec fn total_samples(map: ClassificationMap) -> nat
    decreases map.len()
{
    if map.len() == 0 {
        0
    } else {
        map[0].count + total_samples(map.drop_first())
    }
}

pub open spec fn empty_classifications() -> ClassificationMap {
    seq![]
}


pub proof fn empty_has_zero_samples()
    ensures total_samples(empty_classifications()) == 0
{
}

} // verus!