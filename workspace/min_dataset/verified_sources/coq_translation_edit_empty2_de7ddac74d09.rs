use vstd::prelude::*;

verus! {

pub open spec fn min3(a: nat, b: nat, c: nat) -> nat {
    if a <= b && a <= c { a } else if b <= c { b } else { c }
}


pub open spec fn solve_edit(s1: Seq<nat>, s2: Seq<nat>) -> nat { edit_dist(s1, s2, 0, 0) }

pub open spec fn edit_dist(s1: Seq<nat>, s2: Seq<nat>, i: nat, j: nat) -> nat
    decreases s1.len() - i, s2.len() - j
{
    if i >= s1.len() { (s2.len() - j) as nat }
    else if j >= s2.len() { (s1.len() - i) as nat }
    else if s1[i as int] == s2[j as int] { edit_dist(s1, s2, i + 1, j + 1) }
    else {
        let insert = 1 + edit_dist(s1, s2, i, j + 1);
        let delete = 1 + edit_dist(s1, s2, i + 1, j);
        let replace = 1 + edit_dist(s1, s2, i + 1, j + 1);
        min3(insert, delete, replace)
    }
}


pub proof fn edit_empty2(s: Seq<nat>) ensures solve_edit(s, Seq::empty()) == s.len() { reveal_with_fuel(edit_dist, 2); }

} // verus!