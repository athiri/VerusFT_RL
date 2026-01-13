use vstd::prelude::*;

verus! {

pub open spec fn sorted(s: Seq<nat>) -> bool {
    forall|i: int, j: int| 0 <= i < j < s.len() ==> s[i] <= s[j]
}

pub open spec fn binary_search(s: Seq<nat>, target: nat, lo: nat, hi: nat) -> Option<nat>
    decreases hi - lo
{
    if lo >= hi {
        None
    } else {
        let mid = (lo + hi) / 2;
        if s[mid as int] == target {
            Some(mid)
        } else if s[mid as int] < target {
            binary_search(s, target, (mid + 1) as nat, hi)
        } else {
            binary_search(s, target, lo, mid)
        }
    }
}

pub proof fn search_correct(s: Seq<nat>, target: nat, lo: nat, hi: nat)
    requires sorted(s), lo <= hi, hi <= s.len()
    ensures match binary_search(s, target, lo, hi) {
        Some(i) => lo <= i < hi && s[i as int] == target,
        None => forall|i: nat| lo <= i < hi ==> s[i as int] != target,
    }
    decreases hi - lo
{
    if lo < hi {
        let mid = (lo + hi) / 2;
        if s[mid as int] == target {
        } else if s[mid as int] < target {
            search_correct(s, target, (mid + 1) as nat, hi);
        } else {
            search_correct(s, target, lo, mid);
        }
    }
}

} // verus!
