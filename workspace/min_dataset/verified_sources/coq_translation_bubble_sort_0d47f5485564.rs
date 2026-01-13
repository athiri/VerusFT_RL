use vstd::prelude::*;

verus! {

pub open spec fn swap_at(s: Seq<nat>, i: nat, j: nat) -> Seq<nat>
    recommends i < s.len(), j < s.len()
{
    s.update(i as int, s[j as int]).update(j as int, s[i as int])
}


pub open spec fn bubble_pass(s: Seq<nat>, i: nat) -> Seq<nat>
    recommends i < s.len()
    decreases s.len() - i
{
    if i + 1 >= s.len() {
        s
    } else if s[i as int] > s[(i + 1) as int] {
        bubble_pass(swap_at(s, i, i + 1), i + 1)
    } else {
        bubble_pass(s, i + 1)
    }
}


pub open spec fn one_pass(s: Seq<nat>) -> Seq<nat> {
    if s.len() <= 1 {
        s
    } else {
        bubble_pass(s, 0)
    }
}


pub open spec fn bubble_sort(s: Seq<nat>, passes: nat) -> Seq<nat>
    decreases passes
{
    if passes == 0 || s.len() <= 1 {
        s
    } else {
        bubble_sort(one_pass(s), (passes - 1) as nat)
    }
}

} // verus!