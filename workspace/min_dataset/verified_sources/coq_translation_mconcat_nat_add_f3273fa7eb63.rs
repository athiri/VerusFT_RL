use vstd::prelude::*;

verus! {

pub open spec fn nat_add(a: nat, b: nat) -> nat { a + b }

pub open spec fn nat_add_identity() -> nat { 0 }


pub open spec fn mconcat_nat_add(s: Seq<nat>) -> nat
    decreases s.len()
{
    if s.len() == 0 { nat_add_identity() }
    else { nat_add(s[0], mconcat_nat_add(s.skip(1))) }
}

} // verus!