use vstd::prelude::*;

verus! {

pub open spec fn gen_int_map(outputs: Set<int>, f: spec_fn(int) -> int) -> Set<int> {
    Set::new(|n: int| exists|m: int| outputs.contains(m) && f(m) == n)
}


pub open spec fn gen_int_negate(outputs: Set<int>) -> Set<int> {
    gen_int_map(outputs, |n: int| -n)
}


pub proof fn gen_int_negate_involutive(outputs: Set<int>)
    ensures gen_int_negate(gen_int_negate(outputs)) =~= outputs
{
    assert forall|n: int| gen_int_negate(gen_int_negate(outputs)).contains(n) <==>
        outputs.contains(n) by {
        if gen_int_negate(gen_int_negate(outputs)).contains(n) {
            let m = choose|m: int| gen_int_negate(outputs).contains(m) && -m == n;
            let k = choose|k: int| outputs.contains(k) && -k == m;
            assert(k == n);
        }
        if outputs.contains(n) {
            assert(gen_int_negate(outputs).contains(-n));
            assert(gen_int_negate(gen_int_negate(outputs)).contains(n));
        }
    }
}

} // verus!