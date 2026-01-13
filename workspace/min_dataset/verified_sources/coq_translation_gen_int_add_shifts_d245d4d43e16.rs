use vstd::prelude::*;

verus! {

pub open spec fn gen_int_map(outputs: Set<int>, f: spec_fn(int) -> int) -> Set<int> {
    Set::new(|n: int| exists|m: int| outputs.contains(m) && f(m) == n)
}


pub open spec fn in_int_range(n: int, lo: int, hi: int) -> bool {
    lo <= n && n < hi
}

pub open spec fn choose_int_outputs(lo: int, hi: int) -> Set<int> {
    Set::new(|n: int| in_int_range(n, lo, hi))
}

pub open spec fn gen_int_add(outputs: Set<int>, k: int) -> Set<int> {
    gen_int_map(outputs, |n: int| n + k)
}


pub proof fn gen_int_add_shifts(lo: int, hi: int, k: int)
    ensures
        forall|n: int| gen_int_add(choose_int_outputs(lo, hi), k).contains(n) <==>
            in_int_range(n, lo + k, hi + k)
{
    assert forall|n: int| gen_int_add(choose_int_outputs(lo, hi), k).contains(n) <==>
        in_int_range(n, lo + k, hi + k) by {
        if gen_int_add(choose_int_outputs(lo, hi), k).contains(n) {
            let m = choose|m: int| choose_int_outputs(lo, hi).contains(m) && m + k == n;
            assert(in_int_range(m, lo, hi));
            assert(n == m + k);
        }
        if in_int_range(n, lo + k, hi + k) {
            let m = n - k;
            assert(choose_int_outputs(lo, hi).contains(m));
        }
    }
}

} // verus!