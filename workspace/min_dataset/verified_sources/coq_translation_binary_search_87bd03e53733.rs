use vstd::prelude::*;

verus! {

pub open spec fn binary_search(s: Seq<nat>, target: nat, lo: nat, hi: nat) -> Option<nat>
    decreases hi - lo
{
    if lo >= hi { None }
    else {
        let mid = (lo + (hi - lo) / 2) as nat;
        if s[mid as int] == target { Some(mid) }
        else if s[mid as int] < target { binary_search(s, target, (mid + 1) as nat, hi) }
        else { binary_search(s, target, lo, mid) }
    }
}

} // verus!