use vstd::prelude::*;

verus! {

pub open spec fn insert(x: nat, s: Seq<nat>) -> Seq<nat>
    decreases s.len()
{
    if s.len() == 0 {
        seq![x]
    } else if x <= s[0] {
        seq![x].add(s)
    } else {
        seq![s[0]].add(insert(x, s.skip(1)))
    }
}


pub open spec fn insertion_sort(s: Seq<nat>) -> Seq<nat>
    decreases s.len()
{
    if s.len() == 0 {
        Seq::<nat>::empty()
    } else {
        insert(s[0], insertion_sort(s.skip(1)))
    }
}

pub proof fn insert_length(x: nat, s: Seq<nat>)
    ensures insert(x, s).len() == s.len() + 1
    decreases s.len()
{
    reveal_with_fuel(insert, 2);
    if s.len() == 0 {
    } else if x <= s[0] {
    } else {
        insert_length(x, s.skip(1));
    }
}


pub proof fn sort_length(s: Seq<nat>)
    ensures insertion_sort(s).len() == s.len()
    decreases s.len()
{
    reveal_with_fuel(insertion_sort, 2);
    if s.len() == 0 {
    } else {
        sort_length(s.skip(1));
        insert_length(s[0], insertion_sort(s.skip(1)));
    }
}

} // verus!