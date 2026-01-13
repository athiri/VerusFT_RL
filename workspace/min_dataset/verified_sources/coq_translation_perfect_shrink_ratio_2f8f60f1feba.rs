use vstd::prelude::*;

verus! {

pub open spec fn shrink_ratio(original: nat, final_val: nat) -> nat {
    if original == 0 {
        100
    } else if original <= final_val {
        0
    } else {
        (((original - final_val) * 100) / (original as int)) as nat
    }
}


pub proof fn perfect_shrink_ratio()
    ensures shrink_ratio(100, 0) == 100
{
    assert((100 - 0) * 100 / 100 == 100) by(nonlinear_arith);
}

} // verus!