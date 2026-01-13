use vstd::prelude::*;

verus! {

pub proof fn ex8_add_succ_l(n: nat, m: nat)
    ensures add(n + 1, m) == add(n, m) + 1
{
    assert(add(n + 1, m) == add(n, m) + 1);
}

} // verus!