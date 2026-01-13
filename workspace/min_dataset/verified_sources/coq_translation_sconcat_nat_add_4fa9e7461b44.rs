use vstd::prelude::*;

verus! {

pub open spec fn nat_add(a: nat, b: nat) -> nat { a + b }


pub open spec fn sconcat_nat_add(s: Seq<nat>) -> nat
    decreases s.len()
{
    if s.len() == 0 { 0 }
    else if s.len() == 1 { s[0] }
    else { nat_add(s[0], sconcat_nat_add(s.skip(1))) }
}

} // verus!