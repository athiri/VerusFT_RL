use vstd::prelude::*;

verus! {

pub struct SkipNode {
    pub key: nat,
    pub forward: Seq<nat>,  // forward[i] = index of next node at level i
}

pub struct SkipList {
    pub nodes: Seq<SkipNode>,
    pub max_level: nat,
    pub head: nat,
}


pub open spec fn skip_search_fuel(sl: SkipList, key: nat, level: nat, current: nat, fuel: nat) -> bool
    decreases fuel
{
    if fuel == 0 {
        false
    } else if current >= sl.nodes.len() {
        false
    } else if sl.nodes[current as int].key == key {
        true
    } else if level > 0 {
        // Try lower level
        skip_search_fuel(sl, key, (level - 1) as nat, current, (fuel - 1) as nat)
    } else if sl.nodes[current as int].forward.len() > 0 {
        // Move forward at level 0
        let next = sl.nodes[current as int].forward[0];
        if next < sl.nodes.len() && next != current {
            skip_search_fuel(sl, key, 0, next, (fuel - 1) as nat)
        } else {
            false
        }
    } else {
        false
    }
}

} // verus!