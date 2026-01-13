use vstd::prelude::*;

verus! {

pub struct Label {
    pub name: nat,  // Using nat as label identifier
    pub count: nat,
}

pub struct LabelStats {
    pub labels: Seq<Label>,
    pub total_tests: nat,
}

pub open spec fn add_label(stats: LabelStats, label_id: nat) -> LabelStats {
    LabelStats {
        labels: stats.labels.push(Label { name: label_id, count: 1 }),
        total_tests: stats.total_tests + 1,
    }
}


pub proof fn add_label_increases_count(stats: LabelStats)
    ensures add_label(stats, 0).total_tests == stats.total_tests + 1
{
}

} // verus!