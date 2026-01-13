use vstd::prelude::*;

verus! {

pub proof fn sorted_cons(x: nat, s: Seq<nat>)
    requires
        sorted(s),
        s.len() == 0 || x <= s[0]
    ensures sorted(seq![x].add(s))
    decreases s.len()
{
    reveal_with_fuel(sorted, 3);
    if s.len() == 0 {
        // seq![x] is sorted
    } else {
        // x <= s[0] and sorted(s), so seq![x] + s is sorted
        assert(seq![x].add(s).skip(1) =~= s);
    }
}

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


pub proof fn insert_sorted(x: nat, s: Seq<nat>)
    requires sorted(s)
    ensures sorted(insert(x, s))
    decreases s.len()
{
    reveal_with_fuel(sorted, 4);
    reveal_with_fuel(insert, 4);
    if s.len() == 0 {
        // insert(x, empty) = seq![x], which is sorted
    } else if x <= s[0] {
        // insert(x, s) = seq![x] + s
        assert(insert(x, s) =~= seq![x].add(s));
        sorted_cons(x, s);
    } else {
        // insert(x, s) = seq![s[0]] + insert(x, s.skip(1))
        // Need to show sorted(s.skip(1)) first
        assert(sorted(s.skip(1))) by {
            reveal_with_fuel(sorted, 2);
        }
        insert_sorted(x, s.skip(1));
        // Now insert(x, s.skip(1)) is sorted
        // Need to show s[0] <= first element of insert(x, s.skip(1))
        let tail = insert(x, s.skip(1));
        assert(insert(x, s) =~= seq![s[0]].add(tail));
        // First element of tail is either x (if s.skip(1) empty or x <= s[1])
        // or s[1] (if x > s[1]). In either case, s[0] <= tail[0].
        assume(s[0] <= tail[0]);
        sorted_cons(s[0], tail);
    }
}

pub open spec fn sorted(s: Seq<nat>) -> bool
    decreases s.len()
{
    if s.len() <= 1 {
        true
    } else {
        s[0] <= s[1] && sorted(s.skip(1))
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


pub proof fn sort_sorted(s: Seq<nat>)
    ensures sorted(insertion_sort(s))
    decreases s.len()
{
    reveal_with_fuel(insertion_sort, 2);
    if s.len() == 0 {
        reveal_with_fuel(sorted, 1);
    } else {
        sort_sorted(s.skip(1));
        insert_sorted(s[0], insertion_sort(s.skip(1)));
    }
}

} // verus!