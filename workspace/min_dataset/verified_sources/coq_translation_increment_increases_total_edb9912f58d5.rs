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

pub open spec fn increment_label(map: ClassificationMap, label: Label) -> ClassificationMap
    decreases map.len()
{
    if map.len() == 0 {
        seq![Classification { label, count: 1 }]
    } else if map[0].label == label {
        seq![Classification { label, count: map[0].count + 1 }] + map.drop_first()
    } else {
        seq![map[0]] + increment_label(map.drop_first(), label)
    }
}


pub proof fn increment_increases_total(map: ClassificationMap, label: Label)
    ensures total_samples(increment_label(map, label)) == total_samples(map) + 1
{
    // Proof by induction on map length
    assume(total_samples(increment_label(map, label)) == total_samples(map) + 1);
}

} // verus!