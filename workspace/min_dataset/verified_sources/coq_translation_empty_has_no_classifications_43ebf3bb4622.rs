use vstd::prelude::*;

verus! {

pub struct Classification {
    pub category: nat,
    pub test_index: nat,
}

pub struct ClassifyResult {
    pub classifications: Seq<Classification>,
    pub category_counts: Map<nat, nat>,
}

pub open spec fn empty_classification() -> ClassifyResult {
    ClassifyResult {
        classifications: Seq::empty(),
        category_counts: Map::empty(),
    }
}

pub open spec fn total_classified(result: ClassifyResult) -> nat {
    result.classifications.len() as nat
}


pub proof fn empty_has_no_classifications()
    ensures total_classified(empty_classification()) == 0
{
}

} // verus!