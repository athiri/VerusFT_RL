use vstd::prelude::*;

verus! {

pub open spec fn nat_mul_identity() -> nat { 1 }

pub open spec fn nat_mul(a: nat, b: nat) -> nat { a * b }


pub open spec fn mconcat_nat_mul(s: Seq<nat>) -> nat
    decreases s.len()
{
    if s.len() == 0 { nat_mul_identity() }
    else { nat_mul(s[0], mconcat_nat_mul(s.skip(1))) }
}

} // verus!