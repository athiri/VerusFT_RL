use vstd::prelude::*;

verus! {

pub open spec fn gen_int(seed: nat, size: nat) -> int {
    if size == 0 {
        0
    } else {
        let raw = (seed % (2 * size + 1)) as int;
        raw - size as int
    }
}


pub proof fn gen_int_in_range(seed: nat, size: nat)
    requires size > 0
    ensures -(size as int) <= gen_int(seed, size) <= size as int
{
    let raw = (seed % (2 * size + 1)) as int;
    assert(0 <= raw < (2 * size + 1) as int);
    let result = raw - size as int;
    assert(-(size as int) <= result);
    assert(result <= size as int);
}

} // verus!