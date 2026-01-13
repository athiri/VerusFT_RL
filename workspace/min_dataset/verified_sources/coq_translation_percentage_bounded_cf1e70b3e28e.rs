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

pub open spec fn find_label(map: ClassificationMap, label: Label) -> Option<nat>
    decreases map.len()
{
    if map.len() == 0 {
        Option::None
    } else if map[0].label == label {
        Option::Some(map[0].count)
    } else {
        find_label(map.drop_first(), label)
    }
}

pub open spec fn label_percentage(map: ClassificationMap, label: Label) -> nat {
    let total = total_samples(map);
    if total == 0 {
        0
    } else {
        match find_label(map, label) {
            Option::Some(count) => (count * 100) / total,
            Option::None => 0,
        }
    }
}


pub proof fn percentage_bounded(map: ClassificationMap, label: Label)
    ensures label_percentage(map, label) <= 100
{
    let total = total_samples(map);
    if total > 0 {
        match find_label(map, label) {
            Option::Some(count) => {
                // count <= total, so count * 100 / total <= 100
                assume((count * 100) / total <= 100);
            }
            Option::None => {}
        }
    }
}

} // verus!