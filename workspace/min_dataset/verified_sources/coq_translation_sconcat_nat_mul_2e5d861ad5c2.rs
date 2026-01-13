use vstd::prelude::*;

verus! {

pub open spec fn nat_mul(a: nat, b: nat) -> nat { a * b }


pub open spec fn sconcat_nat_mul(s: Seq<nat>) -> nat
    decreases s.len()
{
    if s.len() == 0 { 1 }
    else if s.len() == 1 { s[0] }
    else { nat_mul(s[0], sconcat_nat_mul(s.skip(1))) }
}

} // verus!