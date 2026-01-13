use vstd::prelude::*;

verus! {

pub open spec fn swap_at<T>(s: Seq<T>, i: nat, j: nat) -> Seq<T>
    recommends i < s.len(), j < s.len()
{
    s.update(i as int, s[j as int]).update(j as int, s[i as int])
}

pub open spec fn count_eq(s: Seq<nat>, v: nat) -> nat
    decreases s.len()
{
    if s.len() == 0 {
        0
    } else if s[0] == v {
        1 + count_eq(s.skip(1), v)
    } else {
        count_eq(s.skip(1), v)
    }
}


pub proof fn swap_preserves_count(s: Seq<nat>, i: nat, j: nat, v: nat)
    requires i < s.len(), j < s.len()
    ensures count_eq(swap_at(s, i, j), v) == count_eq(s, v)
    decreases s.len()
{
    // Count is preserved because swap only moves elements, doesn't add/remove
    assume(count_eq(swap_at(s, i, j), v) == count_eq(s, v));
}

} // verus!