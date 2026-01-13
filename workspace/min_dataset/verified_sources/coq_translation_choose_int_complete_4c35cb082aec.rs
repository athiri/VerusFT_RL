use vstd::prelude::*;

verus! {

pub open spec fn in_int_range(n: int, lo: int, hi: int) -> bool {
    lo <= n && n < hi
}


pub open spec fn choose_int_outputs(lo: int, hi: int) -> Set<int> {
    Set::new(|n: int| in_int_range(n, lo, hi))
}


pub proof fn choose_int_complete(lo: int, hi: int, n: int)
    requires lo <= n && n < hi
    ensures choose_int_outputs(lo, hi).contains(n)
{
}

} // verus!