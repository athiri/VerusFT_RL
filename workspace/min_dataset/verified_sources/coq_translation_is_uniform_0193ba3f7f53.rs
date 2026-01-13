use vstd::prelude::*;

verus! {

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


pub type Label = nat;

pub struct Classification {
    pub label: Label,
    pub count: nat,
}

pub type ClassificationMap = Seq<Classification>;

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


pub open spec fn is_uniform(map: ClassificationMap, tolerance: nat) -> bool
    decreases map.len()
{
    if map.len() <= 1 {
        true
    } else {
        let expected = 100nat / (map.len() as nat);
        let p = label_percentage(map, map[0].label);
        let diff = if p >= expected { p - expected } else { expected - p };
        diff <= tolerance && is_uniform(map.drop_first(), tolerance)
    }
}

} // verus!