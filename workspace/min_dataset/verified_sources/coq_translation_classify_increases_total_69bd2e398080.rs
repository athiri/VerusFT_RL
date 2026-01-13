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

pub open spec fn total_classified(result: ClassifyResult) -> nat {
    result.classifications.len() as nat
}

pub open spec fn classify_test(result: ClassifyResult, category: nat, test_idx: nat) -> ClassifyResult {
    let new_count = if result.category_counts.contains_key(category) {
        result.category_counts[category] + 1
    } else {
        1nat
    };
    ClassifyResult {
        classifications: result.classifications.push(Classification { category, test_index: test_idx }),
        category_counts: result.category_counts.insert(category, new_count),
    }
}


pub proof fn classify_increases_total(result: ClassifyResult, cat: nat, idx: nat)
    ensures total_classified(classify_test(result, cat, idx)) == total_classified(result) + 1
{
}

} // verus!