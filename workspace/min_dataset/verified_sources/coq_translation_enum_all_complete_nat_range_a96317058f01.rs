use vstd::prelude::*;

verus! {

pub open spec fn enum_count_nat_range(n: nat) -> nat {
    n
}


pub proof fn enum_all_nat_range_index(n: nat, i: nat)
    requires i < n
    ensures enum_all_nat_range(n)[i as int] == i
    decreases n
{
    enum_count_eq_len_nat_range(n);
    if n == 0 {
        // vacuously true
    } else if i < n - 1 {
        enum_count_eq_len_nat_range((n - 1) as nat);
        enum_all_nat_range_index((n - 1) as nat, i);
        // The push operation preserves earlier indices
        let prev = enum_all_nat_range((n - 1) as nat);
        assert(prev.len() == (n - 1) as nat);
        assert(enum_all_nat_range(n) == prev.push((n - 1) as nat));
        assert(prev.push((n - 1) as nat)[i as int] == prev[i as int]);
    } else {
        // i == n - 1
        enum_count_eq_len_nat_range((n - 1) as nat);
        let prev = enum_all_nat_range((n - 1) as nat);
        assert(prev.len() == (n - 1) as nat);
        assert(enum_all_nat_range(n) == prev.push((n - 1) as nat));
        assert(prev.push((n - 1) as nat)[(n - 1) as int] == (n - 1) as nat);
    }
}

pub proof fn enum_count_eq_len_nat_range(n: nat)
    ensures enum_count_nat_range(n) == enum_all_nat_range(n).len()
    decreases n
{
    if n == 0 {
        assert(enum_all_nat_range(0) =~= Seq::empty());
        assert(enum_all_nat_range(0).len() == 0);
    } else {
        enum_count_eq_len_nat_range((n - 1) as nat);
        assert(enum_all_nat_range(n).len() == enum_all_nat_range((n - 1) as nat).len() + 1);
    }
}

pub open spec fn enum_all_nat_range(n: nat) -> Seq<nat>
    decreases n
{
    if n == 0 {
        Seq::empty()
    } else {
        enum_all_nat_range((n - 1) as nat).push((n - 1) as nat)
    }
}


pub proof fn enum_all_complete_nat_range(n: nat, k: nat)
    requires k < n
    ensures enum_all_nat_range(n).contains(k)
    decreases n
{
    enum_count_eq_len_nat_range(n);
    enum_all_nat_range_index(n, k);
    assert(enum_all_nat_range(n)[k as int] == k);
}

} // verus!