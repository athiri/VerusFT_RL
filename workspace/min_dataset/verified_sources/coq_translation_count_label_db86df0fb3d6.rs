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


pub open spec fn count_label(stats: LabelStats, label_id: nat) -> nat
    decreases stats.labels.len()
{
    if stats.labels.len() == 0 {
        0
    } else {
        let last = stats.labels.last();
        let rest = LabelStats { labels: stats.labels.drop_last(), total_tests: stats.total_tests };
        if last.name == label_id {
            last.count + count_label(rest, label_id)
        } else {
            count_label(rest, label_id)
        }
    }
}

} // verus!