use vstd::prelude::*;

verus! {

pub type Label = nat;

pub struct Classification {
    pub label: Label,
    pub count: nat,
}

pub type ClassificationMap = Seq<Classification>;


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

} // verus!