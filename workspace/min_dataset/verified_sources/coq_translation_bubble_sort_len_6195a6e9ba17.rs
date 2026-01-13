use vstd::prelude::*;

verus! {

pub open spec fn swap_at(s: Seq<nat>, i: nat, j: nat) -> Seq<nat>
    recommends i < s.len(), j < s.len()
{
    s.update(i as int, s[j as int]).update(j as int, s[i as int])
}


pub proof fn bubble_pass_len(s: Seq<nat>, i: nat)
    requires i < s.len()
    ensures bubble_pass(s, i).len() == s.len()
    decreases s.len() - i
{
    reveal_with_fuel(bubble_pass, 3);
    if i + 1 < s.len() {
        if s[i as int] > s[(i + 1) as int] {
            bubble_pass_len(swap_at(s, i, i + 1), i + 1);
        } else {
            bubble_pass_len(s, i + 1);
        }
    }
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


pub proof fn one_pass_len(s: Seq<nat>)
    ensures one_pass(s).len() == s.len()
{
    if s.len() > 1 {
        bubble_pass_len(s, 0);
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


pub proof fn bubble_sort_len(s: Seq<nat>, passes: nat)
    ensures bubble_sort(s, passes).len() == s.len()
    decreases passes
{
    reveal_with_fuel(bubble_sort, 2);
    if passes > 0 && s.len() > 1 {
        one_pass_len(s);
        bubble_sort_len(one_pass(s), (passes - 1) as nat);
    }
}

} // verus!