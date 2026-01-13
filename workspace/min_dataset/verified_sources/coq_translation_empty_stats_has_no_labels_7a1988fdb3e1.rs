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

pub open spec fn empty_stats() -> LabelStats {
    LabelStats { labels: Seq::empty(), total_tests: 0 }
}


pub proof fn empty_stats_has_no_labels()
    ensures empty_stats().labels.len() == 0
{
}

} // verus!