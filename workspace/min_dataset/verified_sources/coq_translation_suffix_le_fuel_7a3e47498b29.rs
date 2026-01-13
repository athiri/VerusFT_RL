use vstd::prelude::*;

verus! {

pub open spec fn suffix(text: Seq<nat>, start: nat) -> Seq<nat> {
    if start <= text.len() {
        text.skip(start as int)
    } else {
        Seq::empty()
    }
}


pub open spec fn suffix_le_fuel(text: Seq<nat>, i: nat, j: nat, fuel: nat) -> bool
    decreases fuel
{
    if fuel == 0 {
        true  // Default to true for termination
    } else {
        let si = suffix(text, i);
        let sj = suffix(text, j);
        if si.len() == 0 {
            true
        } else if sj.len() == 0 {
            false
        } else if si[0] < sj[0] {
            true
        } else if si[0] > sj[0] {
            false
        } else {
            suffix_le_fuel(text, i + 1, j + 1, (fuel - 1) as nat)
        }
    }
}

} // verus!