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

} // verus!