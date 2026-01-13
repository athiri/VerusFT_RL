use vstd::prelude::*;

verus! {

pub open spec fn enum_count_nat_range(n: nat) -> nat {
    n
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

} // verus!